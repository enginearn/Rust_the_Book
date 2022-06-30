// #![feature(default_free_fn)]
// use std::default::default;
// use core::default::default;

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here.
        println!("{:?}", &self);
    }
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct


// enum Option<T> {
//     None,
//     Some(T),
// }

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        // Coin::Quarter(state: UsState) => {
        //     println!("State quarter from {:?}!", state);
        //     25
        // },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i: i32) => Some(i + 1),
    }
}

#[allow(unused)]
fn main() {
    enum IpAddrKind {
        V4(u8, u8, u8, u8), // V4(String), // V4,
        V6(String),         // V6,
    }

    let some_number = Some(5);
    let some_string = Some("a string");
    
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum: <Option<i8> as Add<i8>>::Output = x + y;
    // let sum: i8 = x + y.unwrap_or(default: 0);
    // println!("{}", sum);

    let res1 = value_in_cents(Coin::Penny);
    let res2 = value_in_cents(Coin::Quarter);
    println!("{}", res1);
    println!("{}", res2);

    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);

    let some_value: Option<i32> = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_value {
        println!("three");
    }
    // value_in_cents(Coin::Quarter(UsState::Alaska));

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let home = IpAddrKind::V4(127, 0, 0, 1);
    // let home = IpAddrKind::V4(String::from("127.0.0.1"));

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddrKind::V6(String::from("::1"));
    // let loopback {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);
    // fn route(ip_kind: IpAddrKind) {}

    let m = Message::Write(String::from("hello"));
    m.call();
}

