pub struct Words {
    with: Vec<String>,
    replace: Vec<String>,
}
impl Default for Words {
    fn default() -> Self {
        Self { 
            with: vec!["with".to_string(), "to".to_string()],
            replace: Default::default()
        }
    }
}
