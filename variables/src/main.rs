fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let tup = (4, 'r', "rtrtr");
    let (num, _, _) = tup;
    println!("the vars are {}", num);
}
