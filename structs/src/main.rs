struct User {
    username: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("Hello, world!");

    let username = String::from("Fo");

    let user1 = User {
        username,
        sign_in_count: 54,
        active: true,
    };

    let user2 = User {
        username: String::from("test"),
        ..user1
    };

    println!("{}", user2.username);
}
