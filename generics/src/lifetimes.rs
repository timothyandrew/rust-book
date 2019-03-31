#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

pub fn main() {
    println!("\n-------------\n  Lifetimes\n-------------\n");

    {
        let x = 5;
        let r = &x;
        println!("{}", r);
    }

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    let s = String::from("fop");
    let excerpt = ImportantExcerpt { part: &s[..] };
    {
        println!("String is: {}", s);
    }
    println!("Excerpt is: {:?}", excerpt);
}
