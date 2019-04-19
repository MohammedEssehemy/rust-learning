mod math_helpers;
fn main() {
    let numbers = vec![1, 3, 4, 8, 9, 4, 13, 5, 13, 6, 3, 9, 1, 9, 5, 3, 3, 9, 8];
    println!("Total: {}", math_helpers::sum(&numbers));
    println!("Mean: {:.2}", math_helpers::mean(&numbers));
    println!("Midian: {:.2}", math_helpers::median(&numbers));
    println!("Mode: {}", math_helpers::mode(&numbers));
}
