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

#![allow(unused)]
fn main() {
    enum IpAddrKind {
        v4(u8, u8, u8, u8), // V4(String), // V4,
        V6(String),         // V6,
    }

enum Option<T> {
    None,
    Some(T),

    let some_number = Some(5);
    let Some_string = Some("a string");

    let absent_number: Option<i32> = None;
}

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

