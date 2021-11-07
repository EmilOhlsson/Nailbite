use std::collections::HashMap;

// TODO This could very likely be merged with Exp
#[derive(Debug, PartialEq, Eq)]
enum Res {
    Integer(i32),
    Nothing,
}

impl Res {
    fn to_integer(self) -> i32 {
        match self {
            Res::Integer(integer) => integer,
            _ => panic!("Unable to convert {:?} to integer", self),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Exp {
    Symbol(String),
    Integer(i32),
    List(Vec<Exp>),
    Program(Vec<Exp>),
}

impl Exp {
    fn to_string(self) -> String {
        match self {
            Exp::Symbol(symbol) => symbol,
            _ => panic!("Unable to convert {:?} to string", self),
        }
    }
}

#[derive(Debug)]
struct ParseState {
    parse_stack: Vec<Exp>,
    parse_buffer: Vec<char>,
    tokens: Vec<String>,
    in_symbol: bool,
}

impl ParseState {
    fn new() -> Self {
        ParseState {
            parse_stack: vec![Exp::Program(Vec::new())],
            parse_buffer: Vec::new(),
            tokens: Vec::new(),
            in_symbol: false,
        }
    }

    fn complete_token(&mut self) {
        if !self.in_symbol {
            return;
        }
        let tok: String = self.parse_buffer.iter().collect();
        if let Some(Exp::List(list)) = self.parse_stack.last_mut() {
            let first = tok.chars().next().unwrap();
            if first.is_digit(10) {
                list.push(Exp::Integer(tok.parse::<i32>().unwrap()))
            } else {
                list.push(Exp::Symbol(tok));
            }
        } else {
            panic!("AAAaargh!");
        }
        self.in_symbol = false;
        self.parse_buffer.clear();
    }

    fn complete_list(&mut self) {
        if let Some(exp) = self.parse_stack.pop() {
            if let Some(Exp::List(v)) = self.parse_stack.last_mut() {
                v.push(exp);
            } else if let Some(Exp::Program(v)) = self.parse_stack.last_mut() {
                v.push(exp);
            } else {
                panic!(
                    "Unable to push expression to list: {:?}",
                    self.parse_stack.last()
                );
            }
        } else {
            panic!("Unexpected ')'");
        }
    }

    fn complete(mut self) -> Exp {
        assert!(self.parse_stack.len() == 1);
        self.parse_stack.pop().unwrap()
    }
}

/// Parse a program into syntax tree
fn parse(input: &str) -> Exp {
    // TODO: Create helpers
    let state = input.chars().fold(ParseState::new(), |mut state, ch| {
        if ch == '(' {
            state.complete_token();
            state.parse_stack.push(Exp::List(Vec::new()));
        } else if ch == ')' {
            state.complete_token();
            state.complete_list();
        } else if ch.is_whitespace() {
            state.complete_token();
        } else {
            state.parse_buffer.push(ch);
            state.in_symbol = true;
        }

        state
    });
    state.complete()
}

struct Env {
    symbols: Vec<HashMap<String, Exp>>,
}

impl Env {
    fn new() -> Env {
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
    fn eval(&mut self, expression: &Exp) -> Res {
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

fn main() {
    let simple = "(* (+ 2 2) (+ 4 4) (- 4 2) 100)";
    let program = parse(simple);
    println!("Program: {:?}", program);
    let mut env = Env::new();
    let result = env.eval(&program);
    println!("Result: {:?}", result);
}

#[test]
fn test_simple_no_nesting() {
    use Exp::*;
    let code = "(+ 1 2 3)";
    let ast = parse(code);
    assert_eq!(
        ast,
        Program(vec![List(vec![
            Symbol("+".to_string()),
            Integer(1),
            Integer(2),
            Integer(3)
        ])])
    );
    let mut env = Env::new();
    let result = env.eval(&ast);
    assert_eq!(result, Res::Integer(6));
}

#[test]
fn test_simple_nesting() {
    use Exp::*;
    let code = "(* 3 (+ 2 1))";
    let ast = parse(code);
    assert_eq!(
        ast,
        Program(vec![List(vec![
            Symbol("*".to_string()),
            Integer(3),
            List(vec![Symbol("+".to_string()), Integer(2), Integer(1)])
        ])])
    );
    let mut env = Env::new();
    let result = env.eval(&ast);
    assert_eq!(result, Res::Integer(9));
}

#[test]
fn test_symbols() {
    let code = "(define a 7)(* a a)";
    let ast = parse(code);
    let mut env = Env::new();
    let result = env.eval(&ast);
    assert_eq!(result, Res::Integer(49));
}
