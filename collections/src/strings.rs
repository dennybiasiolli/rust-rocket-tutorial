pub fn test1() {
    let data = "initial contents";
    let _s = data.to_string();
    // same as
    let _s = "initial contents".to_string();
    // same as
    let _s = String::from("initial contents");

    let hellos = [
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שָׁלוֹם"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];
    for hello in &hellos {
        println!("{hello}, length: {}", hello.len());
        for char in hello.chars() {
            println!("{char}");
        }
    }
}

pub fn test2() {
    let mut s1 = String::from("foo");
    let s2 = "ba";
    s1.push_str(s2);
    s1.push('r');
    println!("s1 is {s1}");
    println!("s2 is {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s3}, {s2}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}