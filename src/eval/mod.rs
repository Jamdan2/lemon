mod env;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Null,
    Int(i32),
    Str(String),
    Bool(bool),
    Return(Box<Object>),
}
