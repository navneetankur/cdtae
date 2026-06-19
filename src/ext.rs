pub trait ContainsStr {
    fn contains_str(&self, input: &str) -> bool;
}
impl ContainsStr for Vec<String> {
    fn contains_str(&self, input: &str) -> bool {
        self.iter().any(|s| s == input)
    }
}

#[cfg(test)]
mod tests {
    use crate::ext::ContainsStr;

    #[test]
    fn t_contains() {
        let words = vec!["apple".to_string(), "banana".to_string(), "mango".to_string()];
        assert!(words.contains_str("apple"));
    }
    #[test]
    fn t_not_contains() {
        let words = vec!["apple".to_string(), "banana".to_string(), "mango".to_string()];
        assert!(!words.contains_str("dog"));
    }
}
