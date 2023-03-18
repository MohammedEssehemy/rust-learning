#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
        unsafe { &*(self.b) }
    }
}

fn main() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!("test1 before - a: {}, b: {}", test1.a(), test1.b());
    println!("test2 before - a: {}, b: {}", test2.a(), test2.b());
    std::mem::swap(&mut test1, &mut test2);
    println!("test1 after - a: {}, b: {}", test1.a(), test1.b());
    println!("test2 after - a: {}, b: {}", test2.a(), test2.b());

}
