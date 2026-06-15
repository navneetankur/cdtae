pub struct Words {
    pub with: Vec<String>,
    pub replace: Vec<String>,
    pub undo: Vec<String>,
    pub redo: Vec<String>,
    pub over: Vec<String>,
}
impl Default for Words {
    fn default() -> Self {
        Self { 
            with: vec!["with".to_string(), "to".to_string()],
            replace: vec!["replace".to_string()],
            undo: vec!["undo".to_string()],
            redo: vec!["redo".to_string()],
            over: vec!["over".to_string()],
        }
    }
}
