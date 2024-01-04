struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // `&self` is an immutable borrow of `self`
    // - reading (`&self`)
    // - mutating (`&mut self`)
    // - consuming (`self`)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!(
        "user2: {} <{}>, active: {}, sign in count: {}",
        user2.username, user2.email, user2.active, user2.sign_in_count
    );

    let black = Color(0, 0, 0);
    println!("black: {} {} {}", black.0, black.1, black.2);
    let origin = Point(0, 0, 0);
    println!("origin: {} {} {}", origin.0, origin.1, origin.2);

    let _subject = AlwaysEqual;

    rectanble_area_example();
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn rectanble_area_example() {
    let scale = 1;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1) // immutable borrow of rect1
    );

    // println!("rect1 is {}", rect1);
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);
    println!("rect1 height is {:#?}", rect1.height);

    // method syntax
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(3);
    dbg!(&square1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
