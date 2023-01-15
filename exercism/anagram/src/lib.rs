use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_len = word.len();
    let word_factor = get_word_factor(word);
    let word_hash = get_word_chars_sorted(&word_factor);
    possible_anagrams //
        .iter()
        .filter_map(|&anagram| {
            let anagram_len = anagram.len();
            if anagram_len != word_len {
                return None;
            }
            let anagram_factor = get_word_factor(anagram);
            if anagram_factor == word_factor {
                return None;
            }

            if get_word_chars_sorted(&anagram_factor) != word_hash {
                return None;
            }
            return Some(anagram);
        })
        .collect()
}

fn get_word_factor(word: &str) -> String {
    word.to_lowercase()
}

fn get_word_chars_sorted(word: &str) -> Vec<char> {
    let mut word_chars = word.chars().collect::<Vec<_>>();
    word_chars.sort_unstable();
    word_chars
}
