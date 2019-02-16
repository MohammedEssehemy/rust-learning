fn main() {
    let sentence = String::from("Hello, world!");
    let my_str = first_word(&sentence);

    println!("{}", my_str);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
