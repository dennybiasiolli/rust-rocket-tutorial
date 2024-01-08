use crate::traits::{Summary, Tweet};

mod generics;
mod lifetimes;
mod traits;

#[derive(Debug)]
struct Point<T, U, V> {
    x: T,
    y: U,
    z: V,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = generics::largest_i32(&number_list);
    println!("{number_list:?}");
    println!("The largest number is {largest}");
    let largest = generics::largest(&number_list);
    println!("{number_list:?}");
    println!("The largest number is {largest}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest = generics::largest_i32(&number_list);
    println!("{number_list:?}");
    println!("The largest number is {largest}");
    let largest = generics::largest(&number_list);
    println!("{number_list:?}");
    println!("The largest number is {largest}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = generics::largest_char(&char_list);
    println!("{char_list:?}");
    println!("The largest char is {}", result);
    let result = generics::largest(&char_list);
    println!("{char_list:?}");
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10, z: 15 };
    println!("{}, {}, {}", integer.x, integer.y, integer.z);
    println!("{integer:?}, {}, {}", integer.x, integer.y);
    let float = Point {
        x: 1.0,
        y: 4.0,
        z: 9.0,
    };
    println!("{float:?}",);
    let mix = Point {
        x: 1,
        y: 4.0,
        z: "a",
    };
    println!("{mix:?}");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    traits::notify(&tweet);

    lifetimes::test1();
    lifetimes::test2();
    lifetimes::test3();
}

// // all together
// use std::fmt::Display;
// fn longest_with_an_announcement<'a, T>(
//     x: &'a str,
//     y: &'a str,
//     ann: T,
// ) -> &'a str
// where
//     T: Display,
// {
//     println!("Announcement! {}", ann);
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
