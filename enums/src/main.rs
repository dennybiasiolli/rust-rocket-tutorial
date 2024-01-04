enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,                       // has no data associated with it at all
    Move { x: i32, y: i32 },    // has named fields, like a struct does
    Write(String),              // includes a single String
    ChangeColor(i32, i32, i32), // includes three i32 values
}
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
}

fn main() {
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    let _m = Message::Quit;
    let _m = Message::Move { x: 1, y: 2 };
    dbg!(&_m);
    let _m = Message::ChangeColor(0, 160, 255);
    let m = Message::Write(String::from("hello"));
    m.call();

    println!("Penny: {} cents", Coin::Penny.value_in_cents());
    println!("Nickel: {} cents", Coin::Nickel.value_in_cents());
    println!("Dime: {} cents", Coin::Dime.value_in_cents());
    println!(
        "Quarter: {} cents",
        Coin::Quarter(UsState::Alaska).value_in_cents()
    );

    let five = Some(5);
    let six = plus_one(five);
    println!("six: {:?}", six);
    let none = plus_one(None);
    println!("none: {:?}", none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        // _ => reroll(),
        _ => (),
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // is the same as:
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
// fn move_player(num_spaces: u8) {}
// fn reroll() {}
