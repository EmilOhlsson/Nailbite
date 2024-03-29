use nailbite::{parse, Env, Expr};

#[test]
fn test_simple_no_nesting() {
    use Expr::*;
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
    assert_eq!(result, Expr::Integer(6));
}

#[test]
fn test_simple_nesting() {
    use Expr::*;
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
    assert_eq!(result, Expr::Integer(9));
}

#[test]
fn test_symbols() {
    let code = "(define a 7)(* a a)";
    let ast = parse(code);
    let mut env = Env::new();
    let result = env.eval(&ast);
    assert_eq!(result, Expr::Integer(49));
}

#[test]
fn test_lambda() {
    assert_eq!(
        nailbite::run("((lambda (x) (* x x)) 3 3)"),
        Expr::Integer(9)
    );
}

#[test]
fn test_local_scope() {
    assert_eq!(nailbite::run("(let ((a 7)) (* a a))"), Expr::Integer(49));
}
