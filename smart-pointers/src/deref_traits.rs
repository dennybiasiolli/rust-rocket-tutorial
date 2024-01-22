use std::ops::Deref;

pub fn traits1() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // or
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

pub fn traits2() {
    let mut x = 5;
    let y = &x;
    // x borrowed, unable to change it here
    // x = 6;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    x = 6;
    assert_eq!(6, x);

    // or
    let mut x = 5;
    let y = Box::new(x);
    // x not borrowed but copied, able to change it here
    x = 6;
    assert_eq!(6, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn traits3() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

pub fn traits4_unrelated() {
    struct MyTest<T> {
        x: T,
    }
    impl<T> MyTest<T> {
        fn new(x: T) -> MyTest<T> {
            MyTest { x }
        }
    }
    impl<T> Deref for MyTest<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.x
        }
    }

    let t1 = 5;
    let t2 = MyTest::new(t1);
    let t3 = &t2;
    assert_eq!(5, t1);
    assert_eq!(5, *t2);
    assert_eq!(5, **t3);
}

pub fn traits5() {
    fn hello(name: &str) {
        println!("Hello, {name}!",);
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
pub fn cleanup1() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created, we are at the end of main.");
}
pub fn cleanup2() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
