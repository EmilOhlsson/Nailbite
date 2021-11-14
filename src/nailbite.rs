pub mod eval;
pub mod expression;
pub mod parsing;

pub use eval::Env;
pub use expression::*;
pub use parsing::parse;

pub fn run(code: &str) -> Expr {
    let ast = parse(code);
    let mut env = Env::new();
    env.eval(&ast)
}
