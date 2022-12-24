use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let brackets: HashMap<_, _> = HashMap::from_iter(vec![('[', ']'), ('{', '}'), ('(', ')')]);
    let matching_brackets = brackets.values().collect::<Vec<_>>();
    let mut expected_matching: Vec<&char> = vec![];
    for char in string.chars() {
        match char {
            c if brackets.contains_key(&c) => {
                expected_matching.push(brackets.get(&c).unwrap());
            }
            c if matching_brackets.contains(&&c) && expected_matching.pop() != Some(&c) => {
                return false;
            }
            _ => {}
        }
    }

    return expected_matching.is_empty();
}
