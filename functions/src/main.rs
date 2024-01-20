fn main() {
    another_function(5, 'h');
    println!("The value of expression is: {}", expression());
    println!(
        "Is 'racecar' a palindrome? {}",
        if is_palinrome("racecar") { "Yes" } else { "No" }
    );

    if true == false {
        overflows();
    }
    loops();
    whiles();
    fors();

    for number in 1..=10 {
        println!("Fibonacci {number} is {}", fibonacci(number));
    }

    external_call();
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expression() -> i32 {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
    y + 1
}

fn is_palinrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}

fn overflows() {
    let mut i: u8 = 0;
    let mut overflows: u8 = 0;
    loop {
        i = i.checked_add(1).unwrap_or_else(|| {
            overflows = overflows.wrapping_add(1);
            if overflows > 5 {
                panic!("Too many overflows!");
            }
            0
        });
        println!("i is {i}!");
    }
}

fn loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn whiles() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn fors() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn external_call() {
    println!("adder::add_two(2): {}", adder::add_two(2));
}
