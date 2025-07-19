#[derive(Debug, Clone)]
pub enum Expr {
    Call(String, Vec<Expr>),
    StringLiteral(String),
    Number(f64),
    Variable(String),
}

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Num(f64),
    Unit,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{s}"),
            Value::Num(n) => write!(f, "{n}"),
            Value::Unit => write!(f, ""),
        }
    }
}