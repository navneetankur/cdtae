pub trait ContainsStr {
    fn contains_str(&self, input: &str) -> bool;
}
impl ContainsStr for Vec<String> {
    fn contains_str(&self, input: &str) -> bool {
        self.into_iter().any(|s| s == input)
    }
}
