// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_words = magazine.iter().fold(HashMap::new(), |mut agg, word| {
        *agg.entry(word).or_insert(0_u32) += 1;
        agg
    });
    note //
        .iter()
        .all(|note_word| match magazine_words.get_mut(note_word) {
            Some(n) if *n > 0 => {
                *n -= 1;
                true
            }
            _ => false,
        })
}
