use std::collections::HashMap;

#[derive(Debug)]
enum MedianValue {
    Even(u8, u8),
    Odd(u8),
}

fn get_median_value(numbers: &Vec<u8>) -> MedianValue {
    let mut _numbers = numbers.clone();
    _numbers.sort();
    let length = _numbers.len();
    let middle = length / 2;
    if length % 2 == 0 {
        let i = middle - 1;
        let j = middle;
        MedianValue::Even(_numbers[i], _numbers[j])
    } else {
        MedianValue::Odd(_numbers[middle])
    }
}

fn get_mode_value(numbers: &Vec<u8>) -> Option<u8> {
    let map = numbers.iter().fold(HashMap::new(), |mut acc, &n| {
        let count = acc.entry(n).or_insert(0);
        *count += 1;
        acc
    });
    let mut max_occurences = 0;
    let mut ret_value: Option<u8> = None;
    for (key, value) in map {
        if value > max_occurences {
            max_occurences = value;
            ret_value = Some(key);
        }
    }
    ret_value
}

pub fn ex1() {
    let numbers: Vec<u8> = vec![2, 1, 4, 5, 2, 7, 3, 2, 5, 8, 9];
    let median = get_median_value(&numbers);
    println!("Median: {:?}", median);
    let mode = get_mode_value(&numbers);
    println!("Mode: {:?}", mode);
}

pub fn ex2() {
    let strings = [
        String::from("first"),
        String::from("Second"),
        String::from("apple"),
        String::from("Oracle"),
        String::from(""),
    ];
    for string in strings {
        match string.chars().next() {
            Some(c) if ['a', 'e', 'i', 'o', 'u'].contains(&c) => println!("{}-hay", string),
            Some(c) => println!("{}-{}ay", &string[1..], c),
            None => println!("{string}"),
        }
    }
}
