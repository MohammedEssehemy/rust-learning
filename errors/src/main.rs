mod errors;

fn main() {
    let f = errors::read_username_from_file().expect("Error: ");
    println!("{:?}", f);
}
