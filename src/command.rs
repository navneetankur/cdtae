pub enum Command {
    Replace(Replace),
    Undo,
    Redo,
    Write,
}
pub struct Replace {
    from: String,
    to: String,
}
