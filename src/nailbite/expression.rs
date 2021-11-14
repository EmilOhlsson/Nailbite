// TODO This could very likely be merged with Exp
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Exp {
    Symbol(String),
    Integer(i32),
    List(Vec<Exp>),
    Program(Vec<Exp>),
    Nothing,
}

impl Exp {
    pub fn to_string(&self) -> &str {
        match self {
            Exp::Symbol(symbol) => symbol,
            _ => panic!("Unable to convert {:?} to string", self),
        }
    }

    pub fn to_integer(&self) -> i32 {
        match self {
            Exp::Integer(integer) => *integer,
            _ => panic!("Unable to convert {:?} to integer", self),
        }
    }

    pub fn into_integer(self) -> i32 {
        self.to_integer()
    }
}
