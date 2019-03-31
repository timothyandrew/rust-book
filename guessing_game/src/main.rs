use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number!");

    loop {
        print!("Input your guess: ");
        io::stdout().flush().expect("Failed to flush");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
