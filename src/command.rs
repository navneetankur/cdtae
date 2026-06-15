#[derive(Debug, PartialEq)]
pub enum Command {
    Replace(Replace),
    Undo,
    Redo,
    Write,
}
#[derive(Debug, PartialEq)]
pub struct Replace {
    from: String,
    to: String,
}
