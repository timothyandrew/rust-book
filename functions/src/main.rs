fn main() {
    let mut s = String::from("Fooadsf");

    let first = first_word(&s);
    println!("{}", first);

    let a = [1,2,3,4,5];
    let slice: &[i32] = &a[1..3];

    println!("{:?}", slice);

    s.clear();
}

fn first_word(s: &String) -> &str {
    let mut word = String::new();


    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            return &s[..i];
        }

        word.push(c)
    }

    &s[..]
}