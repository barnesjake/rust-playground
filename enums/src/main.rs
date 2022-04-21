
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

// fn route(ip_kind: IpAddrKind) {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        //method body defined here
    }
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny   => 1,
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter(state) => {
            println!("The state is: {:?}", state);
            25
        }
    }
}


fn main() {
    println!("Hello, world!");
    
    let home = IpAddr::V4(127, 0, 0 ,1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // following doesn't compile as compiler doens't know how to add i8 to Option<i8>
    // let sum = x + y;

    // option documentation: https://doc.rust-lang.org/std/option/enum.Option.html


    let coin = Coin::Dime;
    println!("value of dime: {}", value_in_cents(coin));

    let coin = Coin::Quarter(UsState::Alaska);
    println!("value of quarter: {:?}", value_in_cents(coin));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?}", five);
    println!("six: {:?}", six);
    println!("none: {:?}", none);

    // concise control flow with if let
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // nicer, less boiler plate
    if let Some(max) = config_max {
        // code within doesn't run if the value doesn't match the pattern, in this example, if config_match isn't a Some
        println!("The maximum is configured to be {}", max);
    }
    if let None = config_max {
        println!("this won't run because config_max is a some: {:?}", config_max);
    }

    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    
    let mut count = 0;
    let coin_without_state = Coin::Dime;
    if let Coin::Quarter(state) = coin_without_state {
    println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("Count: {}", count);



}
