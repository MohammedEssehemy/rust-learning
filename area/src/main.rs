#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn get_area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, another_rect: &Rectangle) -> bool {
        (self.height > another_rect.height) && (self.width > another_rect.width)
    }

    fn square(size: i32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let my_rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The recatngle area is {} square pixels", my_rect.get_area());

    let another_rect = Rectangle {
        width: 25,
        height: 15,
    };
    println!(
        "can my_rect hold another_rect? {}",
        my_rect.can_hold(&another_rect)
    );

    let square = Rectangle::square(10);

    println!("square: {:#?}", square);
}

fn _get_area(rect: &Rectangle) -> i32 {
    rect.width * rect.height
}
