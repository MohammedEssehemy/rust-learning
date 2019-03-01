pub struct Vegetable {
    pub name: String,
    id: i32,
}

impl Vegetable {
    pub fn new(name: &str) -> Vegetable {
        Vegetable {
            name: String::from(name),
            id: 1,
        }
    }
}