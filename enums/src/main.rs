#[derive(Debug)]
enum UsState {
    Alaska,
    Florida,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Quarter(UsState),
}

fn get_coin_type(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Quarter(us_state) => {
            println!("The state is {:?}", us_state);
            25
        }
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Florida);
    let another_coin = Coin::Quarter(UsState::Alaska);
    let penny = Coin::Penny;
    println!("The value is {:?}", get_coin_type(coin));
    println!("The value is {:?}", get_coin_type(penny));
    println!("The value is {:?}", get_coin_type(another_coin));
    // #[derive(Debug)]
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // #[derive(Debug)]
    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }
    // let my_addr = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let my_addr_six = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
    // #[derive(Debug)]
    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }

    // let my_addr = IpAddr::V4(String::from("127.0.0.1"));
    // let my_addr_six = IpAddr::V6(String::from("::1"));
    // println!("{:?} {:?}", my_addr, my_addr_six);
}
