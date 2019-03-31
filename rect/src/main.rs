#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    let other = Rectangle { width: 20, height: 20 };

    println!("{:#?}", rect);
    println!("The area of the rectangle is {}.", rect.area());
    println!("Rect can hold other? {}", rect.can_hold(&other));
}