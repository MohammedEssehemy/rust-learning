use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .to_lowercase()
        .split_terminator(|c: char| !c.is_alphanumeric() && c != '\'')
        .map(|s| s.trim_matches('\''))
        .filter(|s| !s.is_empty())
        .fold(HashMap::new(), |mut acc, e| {
            *acc.entry(e.to_string()).or_default() += 1_u32;
            acc
        })
}
