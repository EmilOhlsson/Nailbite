use super::expression::Expr;
use std::collections::HashMap;

pub struct Env {
    symbols: Vec<HashMap<String, Expr>>,
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

    fn lookup(&self, symbol: &str) -> &Expr {
        for scope in self.symbols.iter().rev() {
            if let Some(expr) = scope.get(symbol) {
                return expr;
            }
        }
        panic!("Symbol {:?} not found", symbol)
    }

    fn define(&mut self, symbol: String, value: Expr) {
        self.symbols.last_mut().unwrap().insert(symbol, value);
    }

    fn begin_scope(&mut self) {
        self.symbols.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.symbols.pop();
    }

    /// Evaluate an expression to a result
    pub fn eval(&mut self, expression: &Expr) -> Expr {
        match expression {
            Expr::Nothing => Expr::Nothing,
            Expr::Program(list) => {
                let mut result: Option<Expr> = None;
                for exp in list {
                    result = Some(self.eval(exp));
                }
                result.unwrap_or(Expr::Integer(0))
            }
            Expr::List(list) => {
                if let Some(op) = list.first() {
                    println!("calling {:?}", op);
                    let mut args = list.iter().skip(1);
                    // TODO Search for operation in environment, and if not found check intrinsics
                    // TODO: Move intrinsics somewhere sensible
                    match op {
                        Expr::Symbol(sym) => match sym.as_str() {
                            "define" => {
                                let symbol = args.next().unwrap().to_string();
                                let value = args.next().unwrap().clone();
                                self.define(symbol.to_string(), value);
                                Expr::Nothing
                            }
                            "+" => Expr::Integer(
                                args.map(|i| self.eval(i)).map(Expr::into_integer).sum(),
                            ),
                            "*" => Expr::Integer(
                                args.map(|i| self.eval(i)).map(Expr::into_integer).product(),
                            ),
                            "-" => Expr::Integer(
                                args.map(|i| self.eval(i))
                                    .map(Expr::into_integer)
                                    .reduce(|a, b| a - b)
                                    .unwrap_or(0),
                            ),
                            "/" => Expr::Integer(
                                args.map(|i| self.eval(i))
                                    .map(Expr::into_integer)
                                    .reduce(|a, b| a / b)
                                    .unwrap_or(0),
                            ),
                            "let" => {
                                let bindings = args.next().unwrap().to_list();
                                self.begin_scope();
                                // local create bindings
                                for expr in bindings {
                                    let list = expr.to_list();
                                    let symbol = list[0].to_string();
                                    self.define(symbol.to_string(), list[1].clone());
                                }

                                // Evaluate body
                                let mut result = Expr::Nothing;
                                for expr in args {
                                    result = self.eval(expr);
                                }
                                self.end_scope();
                                result
                            }
                            "lambda" => {
                                let params = args
                                    .next()
                                    .unwrap()
                                    .to_list()
                                    .iter()
                                    .map(Expr::to_string)
                                    .map(str::to_string)
                                    .collect::<Vec<String>>();
                                let expr = Box::new(args.next().unwrap().clone());
                                Expr::Procedure { params, expr }
                            }
                            // TODO: lookup defined operation
                            _ => panic!("No such operation: {:?}", op),
                        },
                        Expr::List(_) => {
                            if let Expr::Procedure { params, expr } = self.eval(op) {
                                self.begin_scope();
                                for (param, value) in params.iter().zip(args) {
                                    println!("{:?} -> {:?}", param, value);
                                    let evaluated_param = self.eval(value);
                                    self.define(param.to_string(), evaluated_param);
                                }
                                let result = self.eval(&expr);
                                self.end_scope();
                                result
                            } else {
                                panic!("Unable to evalue {:?} as procedure", op)
                            }
                        }
                        _ => todo!(),
                    }
                } else {
                    panic!("Cannot evaluate empty list")
                }
            }
            Expr::Integer(integer) => Expr::Integer(*integer),
            Expr::Symbol(symbol) => {
                let expr = self.lookup(symbol).clone();
                self.eval(&expr)
            }
            Expr::Procedure { params: _, expr: _ } => expression.clone(),
        }
    }
}
