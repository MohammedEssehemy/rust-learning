#![allow(dead_code)]
mod ref_cell;
mod tree;
// fn main() {
//     let x = 5;
//     let y = &x;

//     assert_eq!(5, x);
//     assert_eq!(&5, y);
//     assert_eq!(5, *y);
// }
// // --------------------------------

// fn main() {
//     let x = 5;
//     let y = Box::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }
// // --------------------------------

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// use std::ops::{Deref, DerefMut};
// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<T> DerefMut for MyBox<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// fn hello(name: &str) {
//     println!("Hello, {}!", name);
// }

// fn hello2(name: &mut String) {
//     println!("Hello, {}!", name);
// }

// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);
//     assert_eq!(5, x);
//     assert_eq!(5, *y);

//     let m = MyBox::new(String::from("Rust".a).deref_mut());
//     hello(&m);

//     let mut z = MyBox::new(String::from("Mutable Rust"));
//     hello2(&mut z);

//     println!("{} of {:X} people know binary, the other half doesn't", 1, 12);
// }
// // --------------------------------
// #[derive(Debug)]
// struct CustomSmartPointer {
//     _data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data `{}`!", self._data);
//     }
// }

// fn main() {
//     let _c = CustomSmartPointer {
//         _data: String::from("my stuff"),
//     };
//     let _d = CustomSmartPointer {
//         _data: String::from("other stuff"),
//     };
//     drop(_d);
//     println!("after end of my_drop");
//     println!("CustomSmartPointers created.");
// }
// // --------------------------------


// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     let b = Cons(3, Box::new(a));
//     let c = Cons(4, Box::new(a));
// }
// // --------------------------------


// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// use std::rc::Rc;

// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     let b = Cons(3, Rc::clone(&a));
//     println!("count after creating b = {}", Rc::strong_count(&a));
//     {
//         let c = Cons(4, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&a));
//     }
//     println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// }
// // --------------------------------


// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use List::{Cons, Nil};

// fn main() {
//     let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
// }
// // --------------------------------


// using references, works but not flexible
// enum List<'a> {
//     Cons(i32, &'a List<'a>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let a = Cons(5, &Cons(10, &Nil));
//     let b = Cons(3, &a);
//     let c = Cons(4, &a);
// }

// // --------------------------------

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     let b = Cons(3, Box::new(a));
//     let c = Cons(4, Box::new(a));
// }
// // --------------------------------

// #[derive(Debug)]
// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// impl Drop for List {
//     fn drop(&mut self) {
//         println!("dropping list: {:?}", self);
//     }
// }

// use crate::List::{Cons, Nil};
// use std::rc::Rc;

// fn main() {
//     let a = Cons(5, Rc::new(Cons(10, Rc::new(Nil))));
//     let rc_a = Rc::new(a);
//     println!("count after creating a = {}", Rc::strong_count(&rc_a));
//     let _b = Cons(3, Rc::clone(&rc_a));
//     println!("count after creating b = {}", Rc::strong_count(&rc_a));
//     {
//     let _c = Cons(4, Rc::clone(&rc_a));
//     println!("count after creating c = {}", Rc::strong_count(&rc_a));
//     }
//     println!("count after c goes out of scope = {}", Rc::strong_count(&rc_a));
// }
// // --------------------------------

// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// fn main() {
//     let value = Rc::new(RefCell::new(5));

//     let a = Cons(Rc::clone(&value), Rc::new(Nil));
//     let rc_a = Rc::new(a);

//     let _b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&rc_a));
//     let _c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&rc_a));

//     *(value.borrow_mut()) += 10;

//     println!("_a after = {:?}", rc_a);
//     println!("_b after = {:?}", _b);
//     println!("_c after = {:?}", _c);
// }
// // --------------------------------

// Creating a reference cycle
// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             Cons(_, item) => Some(item),
//             Nil => None,
//         }
//     }
// }

// fn main() {
//     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

//     println!("a initial rc count = {}", Rc::strong_count(&a));
//     println!("a next item = {:?}", a.tail());

//     let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

//     println!("a rc count after b creation = {}", Rc::strong_count(&a));
//     println!("b initial rc count = {}", Rc::strong_count(&b));
//     println!("b next item = {:?}", b.tail());
//     println!("a next item = {:?}", a.tail());

//     if let Some(link) = a.tail() {
//         *link.borrow_mut() = Rc::clone(&b);
//     }

//     println!("b rc count after changing a = {}", Rc::strong_count(&b));
//     println!("a rc count after changing a = {}", Rc::strong_count(&a));
//     // Uncomment the next line to see that we have a cycle;
//     // it will overflow the stack
//     // println!("a next item = {:?}", a.tail());
//     // println!("b next item = {:?}", b.tail());
// }

fn main() {
  tree::try_tree();
}