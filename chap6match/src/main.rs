fn main() {
    println!("Hello, world!");
    let cc = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("the value is {} ",cc);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let v = 0u8;
    match v {
        1 => println!("one"),
        _ => println!("other")
    }
    //if let 只针对某一种模式
    let v1 = Some(12u8);
    if let Some(3) = v1 {
        println!("three")
    }else {
        println!("others")
    }
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Hawaiwa,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 10,
        Coin::Quarter(gstate) => {
            println!("in us {:?} ",gstate);
            5
        } 
    }
}

// option<T> some

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}