use std::iter::Peekable;

pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.is_empty() => "Fine. Be that way!",
        m if is_question(m) && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if is_question(m) => "Sure.",
        m if is_yelling(m) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}

trait IsEmpty {
    fn is_empty(&mut self) -> bool;
}

impl<I: Iterator> IsEmpty for Peekable<I> {
    fn is_empty(&mut self) -> bool {
        self.peek().is_none()
    }
}

fn is_yelling(message: &str) -> bool {
    let mut alphabets = message.chars().filter(|c| c.is_alphabetic()).peekable();
    !alphabets.is_empty() && alphabets.all(|c| c.is_uppercase())
}

// fn is_yelling(message: &str) -> bool {
//     let mut alphabets = message.chars().filter(|c| c.is_alphabetic()).peekable();
//     !alphabets.peek().is_none() && alphabets.all(|c| c.is_uppercase())
// }

fn is_question(message: &str) -> bool {
    message.ends_with("?")
}
