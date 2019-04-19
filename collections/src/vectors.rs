pub fn log() {
    // array
    let x = [2, 4, 6];
    println!("{:#?}", x);

    // vestor
    let v = vec![1, 2, 3];
    println!("{:#?}", v);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // mutable vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:#?}", v);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }
}
