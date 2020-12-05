// fn main() {
// let v1 = vec![1, 2, 3];
// let v1_iter = v1.iter();
// for val in v1_iter {
//     println!("Got: {}", val);
// }
// println!("{:?}", v1);

// let v1 = vec![1, 2, 3];
// {
//     let mut v1_iter = v1.into_iter();
//     let x = v1_iter.next();
//     v1_iter.next();
// }
// println!("{:?}", v1);

// let v1 = vec![1, 2, 3];
// let v1_iter = v1.iter();
// let total: i32 = v1_iter.sum();
// assert_eq!(total, 6);

// let v1: Vec<_> = vec![1, 2, 3];
// let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
// assert_eq!(v2, vec![2, 3, 4]);
// }

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

fn main() {}