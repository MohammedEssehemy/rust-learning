fn main() {
    let number = 6;
    if number < 3 {
        println!("less than three");
    } else if number < 5 {
        println!("less than five");
    } else {
        println!("higher");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
