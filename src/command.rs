use strum::{EnumDiscriminants, EnumString, IntoStaticStr, VariantNames};

#[derive(Debug, PartialEq, EnumDiscriminants, VariantNames)]
#[strum_discriminants(derive(EnumString, IntoStaticStr))]
#[strum_discriminants(strum(serialize_all = "lowercase"))]
pub enum Command<'a> {
    Replace(Replace<'a>),
    Undo,
    Redo,
    Write(&'a str),
}
#[derive(Debug, PartialEq)]
pub struct Replace<'a> {
    pub from: &'a str,
    pub to: &'a str,
}
