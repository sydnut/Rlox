use std::fmt::{Debug, Display};
#[derive(Debug, Clone)]
pub enum Object {
    String(String),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::String(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

impl Object{
    pub fn string(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
        }
    }
}
