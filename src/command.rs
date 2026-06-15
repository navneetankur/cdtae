use strum::{EnumDiscriminants, VariantNames};

#[derive(Debug, PartialEq, EnumDiscriminants, VariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum Command<'a> {
    Replace(Replace<'a>),
    Undo,
    Redo,
    Write,
}
#[derive(Debug, PartialEq)]
pub struct Replace<'a> {
    pub from: &'a str,
    pub to: &'a str,
}
