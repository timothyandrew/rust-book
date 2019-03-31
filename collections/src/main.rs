use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let i = String::from("foo");

    {
        let mut v: Vec<i32> = vec![1, 2, 3];
        let sv = vec![&i];
        let x: [i32; 4] = [1, 2, 3, 4];

        v.push(15);

        for i in &v {
            println!("v: {}", i);
        }

        println!("{:?}", sv);
        println!("{}", v.last().expect("Failed option"));
    }

    println!("{}", i);

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    let mut m: HashMap<String, String> = HashMap::new();
    m.insert(String::from("Blue"), String::from("Test"));

    let s1 = String::from("test");
    let s2 = String::from("test2");
    let m: HashSet<&String> = vec![&s1, &s2].iter().cloned().collect();
    println!("{:?}", m);
    println!("{:?}", s1);
}
