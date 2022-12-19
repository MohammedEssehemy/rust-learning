use std::fmt::Display;

struct Bottle(u32);

impl Bottle {
    fn apply_action(&self) -> Self {
        Self(if self.0 == 0 { 99 } else { self.0 - 1 })
    }

    fn action_msg(&self) -> &str {
        match self.0 {
            0 => "Go to the store and buy some more",
            1 => "Take it down and pass it around",
            _ => "Take one down and pass it around",
        }
    }
}

impl Display for Bottle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "no more bottles of juice"),
            1 => write!(f, "{} bottle of juice", self.0),
            n => write!(f, "{} bottles of juice", n),
        }
    }
}

fn upper_first(sentence: String) -> String {
    let mut chars = sentence.chars();
    chars
        .next()
        .unwrap_or_default()
        .to_uppercase()
        .chain(chars)
        .collect()
}

pub fn verse(n: u32) -> String {
    let bottles = Bottle(n);
    format!(
        "{} on the wall, {}.\n{}, {} on the wall.\n",
        upper_first(bottles.to_string()),
        bottles,
        bottles.action_msg(),
        bottles.apply_action()
    )
    .to_string()
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(|n| verse(n))
        .collect::<Vec<_>>()
        .join("\n")
}
