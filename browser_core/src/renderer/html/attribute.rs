use alloc::string::String;

// <tag <name>=<value>>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}