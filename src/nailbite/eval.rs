use super::expression::Exp;
use std::collections::HashMap;

pub struct Env {
    symbols: Vec<HashMap<String, Exp>>,
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

impl Env {
    pub fn new() -> Env {
        Env {
            symbols: vec![HashMap::new()],
        }
    }

    fn lookup(&self, symbol: &str) -> &Exp {
        for scope in self.symbols.iter().rev() {
            if let Some(expr) = scope.get(symbol) {
                return expr;
            }
        }
        panic!("Symbol {:?} not found", symbol)
    }

    fn define(&mut self, symbol: String, value: Exp) {
        self.symbols.last_mut().unwrap().insert(symbol, value);
    }

    /// Evaluate an expression to a result
    pub fn eval(&mut self, expression: &Exp) -> Exp {
        match expression {
            Exp::Nothing => Exp::Nothing,
            Exp::Program(list) => {
                let mut result: Option<Exp> = None;
                for exp in list {
                    result = Some(self.eval(exp));
                }
                result.unwrap_or(Exp::Integer(0))
            }
            Exp::List(list) => {
                if let Some(op) = list.first() {
                    println!("calling {:?}", op);
                    let mut args = list.iter().skip(1);
                    // TODO Search for operation in environment, and if not found check intrinsics
                    match op {
                        Exp::Symbol(sym) => match sym.as_str() {
                            "define" => {
                                let symbol = args.next().unwrap().to_string();
                                let value = args.next().unwrap().clone();
                                self.define(symbol.to_string(), value);
                                Exp::Nothing
                            }
                            "+" => Exp::Integer(
                                args.map(|i| self.eval(i)).map(Exp::into_integer).sum(),
                            ),
                            "*" => Exp::Integer(
                                args.map(|i| self.eval(i)).map(Exp::into_integer).product(),
                            ),
                            "-" => Exp::Integer(
                                args.map(|i| self.eval(i))
                                    .map(Exp::into_integer)
                                    .reduce(|a, b| a - b)
                                    .unwrap_or(0),
                            ),
                            "/" => Exp::Integer(
                                args.map(|i| self.eval(i))
                                    .map(Exp::into_integer)
                                    .reduce(|a, b| a / b)
                                    .unwrap_or(0),
                            ),
                            _ => panic!("No such operation: {:?}", op),
                        },
                        _ => todo!(),
                    }
                } else {
                    panic!("Cannot evaluate empty list")
                }
            }
            Exp::Integer(integer) => Exp::Integer(*integer),
            Exp::Symbol(symbol) => {
                let expr = self.lookup(symbol).clone();
                self.eval(&expr)
            }
        }
    }
}
