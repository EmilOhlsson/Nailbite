mod nailbite;

fn main() {
    let simple = "(* (+ 2 2) (+ 4 4) (- 4 2) 100)";
    let program = nailbite::parse(simple);
    println!("Program: {:?}", program);
    let mut env = nailbite::Env::new();
    let result = env.eval(&program);
    println!("Result: {:?}", result);
}
