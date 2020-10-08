// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }


// fn main() {
//         let r;

//         {
//             let x = 5;
//             r = &x;
//         }

//         println!("r: {}", r);
// }


// fn main() {
//     let string1 = String::from("abcd");
//     let result;
//     {
//         let string2 = String::from("xyzsss");

//         result = longest(&string1, &string2);
//     }
//     println!("The longest string is {}", result);
// }

// fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
//     if str1.len() > str2.len() {
//         str1
//     } else {
//         str2
//     }
// }
use std::fmt::Debug;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str, help: &str) -> &str {
        println!("Attention please: {}, {}", announcement, help);
        self.part
    }
}

fn longest_with_an_announcement<'a, T: Debug >( x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Announcement! {:?}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    let x =  String::from("Cal");
    let y =  String::from("l");
    let s: &'static str = "I have a static lifetime.";
    longest_with_an_announcement(&x, &y, &i);
    println!("{}, {}, {}", i.part, i.level(), i.announce_and_return_part(&x, &y));
}

// lifetime elision rules
// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }