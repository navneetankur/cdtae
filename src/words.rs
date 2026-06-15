pub struct Words {
    pub with: Vec<String>,
    pub replace: Vec<String>,
}
impl Default for Words {
    fn default() -> Self {
        Self { 
            with: vec!["with".to_string(), "to".to_string()],
            replace: vec!["replace".to_string()],
        }
    }
}
