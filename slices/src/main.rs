fn main() {
    let mut my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("the first word is: {word}");
    let word = first_word(&my_string[..]);
    println!("the first word is: {word}");
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    println!("the first word is: {word}");

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("the first word is: {word}");
    let word = first_word(&my_string_literal[..]);
    println!("the first word is: {word}");

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    let word2 = second_word(&my_string);
    // my_string.clear(); // error!
    println!("the first word is: {word}");
    println!("the second word is: {word2}");
    my_string.clear();

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut start_index: usize = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if start_index == 0 {
                start_index = i + 1;
            } else {
                return &s[start_index..i];
            }
        }
    }

    &s[start_index..]
}
