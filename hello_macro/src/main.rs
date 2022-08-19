use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
                println!("{}", $x);
            )*
            temp_vec
        }
    };
}


fn main() {
    vec!(2,3,4);
    Pancakes::hello_macro();
}
