mod nailbite;

use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut env = nailbite::Env::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut buffer)?;
        let result = nailbite::run_with_env(&mut env, &buffer);
        println!("=> {:?}", result);
    }
}
