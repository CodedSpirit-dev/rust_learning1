use std::collections::HashMap;
pub fn count_words(text: &str) -> HashMap<String, u32> {
    let mut frequencies = HashMap::new();

    for word in text.trim().to_lowercase().split_whitespace() {
        let counter = frequencies.entry(word.to_string()).or_insert(0);
        *counter += 1;
    }

    frequencies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words() {
        let text = "hello world hello";
        let frequencies = count_words(text);

        assert_eq!(frequencies.get("hello"), Some(&2));
        assert_eq!(frequencies.get("world"), Some(&1));
    }

    #[test]
    fn test_ignore_case() {
        let text = "Hello hello";
        let frequencies = count_words(text);

        assert_eq!(frequencies.get("hello"), Some(&2));
    }

    #[test]
    fn test_trim_whitespace() {
        let text = "  hello   world  ";
        let frequencies = count_words(text);

        assert_eq!(frequencies.get("hello"), Some(&1));
        assert_eq!(frequencies.get("world"), Some(&1));
    }

    #[test]
    fn test_empty_text() {
        let text = "";
        let frequencies = count_words(text);

        assert!(frequencies.is_empty());
    }
}
