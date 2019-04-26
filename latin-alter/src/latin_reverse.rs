pub fn pig_latin(word: &str) -> String {
    let mut result = String::from("");
    let first_char = word.to_lowercase().chars().next();
    // println!("first char: {}", first_char.unwrap());
    match first_char {
        None => {
            panic!("No Caharacter");
        }
        Some('a') | Some('e') | Some('i') | Some('o') | Some('u') => {
            result.push_str(word);
            result.push_str("-hay");
        }
        _ => {
            result.push_str(&word.chars().skip(1).collect::<String>()[..]);
            result.push('-');
            result.push(first_char.unwrap());
            result.push_str("ay");
        }
    }
    println!("word: {}", result);
    return result;
}
