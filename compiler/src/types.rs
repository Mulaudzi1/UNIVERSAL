#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Text,
    Number,
    Decimal,
    Boolean,
    Date,
    Time,
    DateTime,
    Money,
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Entity(String),
    Optional(Box<Type>),
    Result(Box<Type>, Box<Type>),
    Error,
    Unknown,
    Void,
}

pub fn builtin(name: &str) -> Option<Type> {
    Some(match name {
        "Text" => Type::Text, "Number" => Type::Number, "Decimal" => Type::Decimal, "Boolean" => Type::Boolean,
        "Date" => Type::Date, "Time" => Type::Time, "DateTime" => Type::DateTime, "Money" => Type::Money,
        "Error" => Type::Error, _ => return None,
    })
}
