#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Int,
    Uint,
    Float,
    Double,
    Void,
    User,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ImportType {
    Mod,
    Star,
    Multiple(Vec<String>),
}
