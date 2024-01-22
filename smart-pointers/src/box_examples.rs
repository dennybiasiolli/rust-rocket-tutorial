pub fn box1() {
    let b = Box::new(5);
    println!("b = {}", b);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// #[derive(Debug)]
// enum List2<'a> {
//     Cons(i32, &'a List2<'a>),
//     Nil,
// }

pub fn box2() {
    let list1 = List::Cons(1,
        Box::new(List::Cons(2,
            Box::new(List::Cons(3,
                Box::new(List::Nil))))));
    println!("list1 = {:?}", list1);

    // let list2: List2<'_> = List2::Cons(1,
    //     &List2::Cons(2,
    //         &List2::Cons(3,
    //             &List2::Nil)));
    // println!("list2 = {:?}", list2);
}
