use unicode_segmentation::UnicodeSegmentation;

pub fn bytes_chars_graphemes(s: &str) {
    println!("{}, {} bytes, {} chars, {} graphemes", s, s.len(), s.chars().count(), s.graphemes(true).count());
}