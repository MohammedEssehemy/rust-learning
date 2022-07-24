fn main() {
    let sentence = String::from("Hello world!");
    let hello = first_word(&sentence);
    // sentence.clear(); // error!

    println!("The first word is: {}", hello);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
