// TODO This could very likely be merged with Exp

#[derive(Debug, PartialEq, Eq)]
pub enum Res {
    Integer(i32),
    Nothing,
}

impl Res {
    pub fn to_integer(self) -> i32 {
        match self {
            Res::Integer(integer) => integer,
            _ => panic!("Unable to convert {:?} to integer", self),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Exp {
    Symbol(String),
    Integer(i32),
    List(Vec<Exp>),
    Program(Vec<Exp>),
}

impl Exp {
    pub fn to_string(self) -> String {
        match self {
            Exp::Symbol(symbol) => symbol,
            _ => panic!("Unable to convert {:?} to string", self),
        }
    }
}
