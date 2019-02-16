fn main() {
    println!("Hello, world!");
    let x = another_function("Essehemy");
    println!("return {:?}", x );
}

fn another_function(name: &str) -> i32 {
    println!("Hello, {}!", name);
    5
}
