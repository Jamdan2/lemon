
pub enum Obj {
    Null,
    Int(i32),
    Str(String),
    Bool(bool),
    Return(Box<Obj>),
}
