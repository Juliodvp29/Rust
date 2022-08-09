

fn main() {
    println!("Hello, world!");

let _four = IpAddrKind::V4;
let _six = IpAddrKind::V6;
route(_four);

let _home = IpAddr::V4(String::from("127.0.0.1"));
let _loopback = IpAddr::V6(String::from("::1"));



}

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
// let home = IpAddr::V4(127, 0, 0, 1);
// let loopback = IpAddr::V6(String::from("::1"));

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("IPv4"),
        IpAddrKind::V6 => println!("IPv6"),
    }
}


// struct  Ipv4Adrr {
//     addr: String,

// }

// struct Ipv6Adrr {
//     addr: String,
// }

// enum _IpAddr {
//     V4(Ipv4Adrr),
//     V6(Ipv6Adrr),
// }


// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// struct QuitMessage;
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }

// struct WriteMessage(String);
// struct ChangeColorMessage(i32, i32, i32);

// impl Message {
//     fn call(&self) {
//         println!("Hello");
//     }
// }

// let m = Message::Write(String::from("hello"));
// m.call();





// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };


