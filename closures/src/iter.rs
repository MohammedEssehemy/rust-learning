pub fn _try_iter() {
    println!("--------------------");
    println!("try_iter");
    println!("--------------------");



    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
    println!("{:?}", v1);

    let v1 = vec![1, 2, 3];
    {
        let mut v1_iter = v1.clone().into_iter();
        let _x = v1_iter.next();
        v1_iter.next();
    }
    println!("{:?}", v1);

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    let v1: Vec<_> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(| x | x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
