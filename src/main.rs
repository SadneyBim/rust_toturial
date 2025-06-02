use std::fmt::format;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

impl UsState {
    fn existed_in(&self, year:u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959
        }
    }
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{}",describe_state_quarter(state).unwrap_or("None".to_string()));
            25
        }
    }
}

fn puls_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}

fn describe_state_quarter(state: UsState) -> Option<String> {

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let coin = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("Hello, world! {coin}");

    let five = Some(5);
    let six = puls_one(five);
    let none = puls_one(None);

    Some(format!("there are three values {five:?}, {six:?}, {none:?}"));
}
