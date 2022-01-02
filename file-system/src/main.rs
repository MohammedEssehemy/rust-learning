use std::fs;
fn main() {
    read_dir_recursive(".");
}

fn read_dir_recursive(path: &str) {
    let files = fs::read_dir(path).unwrap();
    for dir_entry in files {
        let file_path = dir_entry.unwrap().path();
        if file_path.is_dir() {
            read_dir_recursive(file_path.to_str().unwrap());
        } else {
            println!("{:?}", file_path);
            match fs::read_to_string(&file_path) {
                Ok(str) => {
                    println!("{}", str);
                },
                Err(e) => {
                    println!("Error happened, {:?}", e.to_string())
                }
            }
        }
    }
}
