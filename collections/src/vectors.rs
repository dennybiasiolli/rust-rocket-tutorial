pub fn test1() {
    let _v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // will panic if index is out of bounds
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let third: Option<&i32> = v.get(2);
    if let Some(third) = third {
        println!("The third element is {third}");
    } else {
        println!("There is no third element.");
    }
}

pub fn test2() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6); // will not work, because of immutable borrow of first
    println!("The first element is: {first}");
    v.push(6);
}

pub fn test3() {
    // iterate over immutable references
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // iterate over mutable references
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{i}");
    }

    let v = vec![100, 32, 57];
    for (index, value) in v.iter().enumerate() {
        println!("v[{index}]: {value}");
    }
}

pub fn test4() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => println!("Int: {value}"),
            SpreadsheetCell::Float(value) => println!("Float: {value}"),
            SpreadsheetCell::Text(value) => println!("Text: {value}"),
        }
    }
}