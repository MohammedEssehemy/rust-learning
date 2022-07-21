#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}


pub fn try_shoe() {
    println!("--------------------");
    println!("try_shoe");
    println!("--------------------");



    let shoes = vec![
    Shoe {size: 13, style: "test".to_owned()}, 
    Shoe {size: 15, style: "test3".to_owned()}
    ];
    println!("All shoes: {:?}", shoes);
    let my_size = 15;
    println!("My size is {}, Which ones are on my size? {:?}", my_size, shoes_in_my_size(shoes, my_size));
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

