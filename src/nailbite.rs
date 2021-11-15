pub mod eval;
pub mod expression;
pub mod parsing;

pub use eval::Env;
pub use expression::*;
pub use parsing::parse;

#[allow(dead_code)]
pub fn run(code: &str) -> Expr {
    let ast = parse(code);
    let mut env = Env::new();
    env.eval(&ast)
}

pub fn run_with_env(env: &mut Env, code: &str) -> Expr {
    let ast = parse(code);
    env.eval(&ast)
}
