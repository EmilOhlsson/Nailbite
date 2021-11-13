use super::expression::{Exp, Res};
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
    pub fn eval(&mut self, expression: &Exp) -> Res {
        match expression {
            Exp::Program(list) => {
                let mut result: Option<Res> = None;
                for exp in list {
                    result = Some(self.eval(exp));
                }
                result.unwrap_or(Res::Integer(0))
            }
            Exp::List(list) => {
                if let Some(op) = list.first() {
                    println!("calling {:?}", op);
                    let mut args = list.iter().skip(1);
                    // TODO Search for operation in environment, and if not found check intrinsics
                    match op {
                        Exp::Symbol(sym) => match sym.as_str() {
                            "define" => {
                                let symbol = args.next().unwrap().clone().to_string();
                                let value = args.next().unwrap().clone();
                                self.define(symbol, value);
                                Res::Nothing
                            }
                            "+" => {
                                Res::Integer(args.map(|i| self.eval(i)).map(Res::to_integer).sum())
                            }
                            "*" => Res::Integer(
                                args.map(|i| self.eval(i)).map(Res::to_integer).product(),
                            ),
                            "-" => Res::Integer(
                                args.map(|i| self.eval(i))
                                    .map(Res::to_integer)
                                    .reduce(|a, b| a - b)
                                    .unwrap_or(0),
                            ),
                            "/" => Res::Integer(
                                args.map(|i| self.eval(i))
                                    .map(Res::to_integer)
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
            Exp::Integer(integer) => Res::Integer(*integer),
            Exp::Symbol(symbol) => {
                let expr = self.lookup(symbol).clone();
                self.eval(&expr)
            }
        }
    }
}
