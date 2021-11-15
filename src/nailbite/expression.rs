#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Bool(bool),
    Symbol(String),
    Integer(i32),
    List(Vec<Expr>),
    Program(Vec<Expr>),
    Nothing,
    Procedure {
        params: Vec<String>,
        expr: Box<Expr>,
    },
}

impl Expr {
    pub fn to_bool(&self) -> bool {
        match self {
            Expr::Bool(val) => *val,
            _ => panic!("Unable to convert {:?} to bool", self),
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Expr::Symbol(symbol) => symbol,
            _ => panic!("Unable to convert {:?} to string", self),
        }
    }

    pub fn to_integer(&self) -> i32 {
        match self {
            Expr::Integer(integer) => *integer,
            _ => panic!("Unable to convert {:?} to integer", self),
        }
    }

    pub fn into_integer(self) -> i32 {
        self.to_integer()
    }

    pub fn to_list(&self) -> &Vec<Expr> {
        match self {
            Expr::List(list) => list,
            _ => panic!("Unable to convert to list"),
        }
    }
}
