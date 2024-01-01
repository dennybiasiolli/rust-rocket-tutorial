const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);


    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // let mut i: u8 = 255;
    // println!("The value of i is: {i}");
    // i = i + 1; // overflow in debug mode, 0 in release mode
    // println!("The value of i is: {i}");

    // let mut i: u8 = 255;
    // println!("The value of i is: {i}");
    // i = u8::wrapping_add(i, 1); // always 0
    // println!("The value of i is: {i}");

    let i: u8 = 255;
    println!("The value of i is: {i}");
    match u8::checked_add(i, 2) {
        Some(i) => println!("The value of i is: {i}"),
        None => println!("Overflow!"),
    };
    println!("The value of i is: {i}");

    let i: f32 = 0.1 + 0.2;
    println!("The value of i is: {i}");
    println!("{}", 0.1 + 0.2);
    println!("{}", 0.1f32 + 0.2f32);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of (x, y, z) is: ({x}, {y}, {z})");

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("The value of months[0] is: {}", months[0]);
}
