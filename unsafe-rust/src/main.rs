// use std::slice;
// const HELLO_WORLD: &str = "Hello, world!";
// fn main() {
//     println!("name is: {}", HELLO_WORLD);
//     let mut num = 5;


//     let r1 = &num as *const i32;
//     let r2 = &mut num as *mut i32;

//     unsafe {
//         println!("r1 is: {}", *r1);
//         println!("r2 is: {}", *r2);
//     }

//     unsafe fn dangerous() {
//         println!("test dangerous.");
//     }

//     unsafe {
//         dangerous();
//     }

//     let mut vector = vec![1, 2, 3, 4, 5, 6];
//     let (_left, _right) = split_at_mut(&mut vector, 3);
//     println!("{:?} {:?}", _left, _right);

//     let address = 0x01234usize;
//     let r = address as *mut i32;

//     // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 1000) };

//     // println!("{:?}", values);
//     let abs_i = unsafe { abs(-3) };
//     println!("Absolute value of -3 according to C: {}", abs_i);
// }


// extern "C" {
//     fn abs(input: i32) -> i32;
// }

// // fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
// //     let len = values.len();
// //     assert!(mid <= len);
// //     (&mut values[..mid], &mut values[mid..])
// // }


// fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = values.len();
//     let ptr = values.as_mut_ptr();
//     assert!(mid <= len);
//     unsafe {
//         (
//             slice::from_raw_parts_mut(ptr, mid),
//             slice::from_raw_parts_mut(ptr.add(mid), len - mid),
//         )
//     }
// }

// #[no_mangle]
// pub extern "C" fn call_from_c() {
//     println!("Just called a Rust function from C!");
// }

// trait Pilot {
//     fn fly(&self);
// }

// trait Wizard {
//     fn fly(&self);
// }

// struct Human;

// impl Pilot for Human {
//     fn fly(&self) {
//         println!("This is your captain speaking.");
//     }
// }

// impl Wizard for Human {
//     fn fly(&self) {
//         println!("Up!");
//     }
// }

// impl Human {
//     fn fly(&self) {
//         println!("*waving arms furiously*");
//     }
// }


// trait Animal {
//     fn baby_name() -> String;
// }

// struct Dog;

// impl Dog {
//     fn baby_name() -> String {
//         String::from("Spot")
//     }
// }

// impl Animal for Dog {
//     fn baby_name() -> String {
//         String::from("puppy")
//     }
// }

// fn main() {
//     let person = Human;
//     Pilot::fly(&person);
//     Wizard::fly(&person);
//     person.fly();
//     println!("A baby dog is called a {}", Dog::baby_name());
//     println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
// }




// trait OutlinePrint: fmt::Display {
//     fn outline_print(&self) {
//         let output = self.to_string();
//         let len = output.len();
//         println!("{}", "*".repeat(len + 4));
//         println!("*{}*", " ".repeat(len + 2));
//         println!("* {} *", output);
//         println!("*{}*", " ".repeat(len + 2));
//         println!("{}", "*".repeat(len + 4));
//     }
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl OutlinePrint for Point {}

// use std::fmt;

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

// fn main() {
//     let p = Point { x: 1, y: 3 };
//     p.outline_print();
// }


use std::{fmt, ops::Deref};

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    w.iter().for_each(|f| {
        println!("{}", f);
    });
    println!("w = {}", w);
}
