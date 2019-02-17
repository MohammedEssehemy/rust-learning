#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    println!("Hello, world!");
    let my_user = build_user(String::from("test@test.com"), String::from("my_username"));
    println!("{:#?}", my_user);
    let user2 = User {
        email: String::from("another@example.com"),
        // username: String::from("anotherusername567"),
        ..my_user
    };
    println!("{:#?}", user2);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
