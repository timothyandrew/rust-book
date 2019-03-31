#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


fn main() {
    let home = IpAddr::V4(String::from("1782f"));

    let something = Some(String::from("HELLO!"));
    let nothing = None;

    print(something);
    print(nothing);

    println!("{:?}", home);
}

fn print(val: Option<String>) {
    match val {
        Some(s) => println!("String exists: {}", s),
        None => println!("none"),
    }
}
