enum Command {
    Replace(Replace),
    Undo,
    Redo,
    Write,
}
struct Replace {
    from: String,
    to: String,
}
