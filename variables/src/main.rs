fn main() {
    // Immutability
    let mut x = 5;
    println!("{}", x);
    
    x = 6;
    println!("{}", x);

    // Constants
    const MAX_POINTS: u32 = 100000;
    println!("{}", MAX_POINTS);

    let guess: i32 = "42".parse().expect("NaN");
    println!("{}", guess);

    let n = 0b1111_1111;
    println!("{}", (n == 0xff));

    let z = '✅';
    println!("{}", z);

    let tup: (i32, f64, &str) = (500, 6.4, "foobar");
    println!("{}", tup.2);

    let arr: [i32; 4] = [1,2,3,4];
    println!("{:?}", arr);

    another(0);
}

fn another(_i: i32) {
    println!("AN");
}
