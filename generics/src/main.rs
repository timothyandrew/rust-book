mod lifetimes;
mod traits;

use traits::Summary;

fn notify<T, U>(item: T, item2: U) -> T
where
    T: Summary,
    U: Summary,
{
    println!("{}", item.summarize());
    println!("{}", item2.summarize());

    item
}

fn largest_int<T: PartialOrd + Copy>(collection: &[T]) -> T {
    let mut largest = collection[0];

    for &item in collection.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: PartialOrd>(collection: &[T]) -> &T {
    let mut largest: &T = &collection[0];

    for item in collection.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let tweet = traits::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably"),
        reply: false,
        retweet: false,
    };

    let article = traits::NewsArticle {
        headline: String::from("headline"),
        location: String::from("loc"),
        author: String::from("au"),
        content: String::from("contascfasdlkj"),
    };

    notify(tweet, article);

    let s1 = String::from("First");
    let s2 = String::from("Second");
    let s3 = String::from("Third");
    let s4 = String::from("Fourth");

    let c = vec![s1, s2, s3, s4];
    let i = vec![1, 2, 3, 4, 70, 4, 5, 6, 7, 12];

    println!("Largest string: {:?}", largest(&c[..]));
    println!("Largest int: {:?}", largest_int(&i[..]));
    println!("Hello, world! {:?}", c);

    traits::foo();
    lifetimes::main();
}
