use std::{
    error::Error,
    fs::{self, File},
    io,
};

fn main() -> Result<(), Box<dyn Error>> {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    // {
    //     let greeting_file_result = File::open("files/hello.txt");
    //     let _greeting_file = match greeting_file_result {
    //         Ok(file) => file,
    //         Err(error) => match error.kind() {
    //             ErrorKind::NotFound => match File::create("files/hello.txt") {
    //                 Ok(fc) => fc,
    //                 Err(e) => panic!("Problem creating the file: {:?}", e),
    //             },
    //             other_error => panic!("Problem opening the file: {:?}", other_error),
    //         },
    //     };
    // }
    // is the same as
    // {
    //     let _greeting_file = File::open("files/hello.txt").unwrap_or_else(|error| {
    //         if error.kind() == ErrorKind::NotFound {
    //             File::create("files/hello.txt").unwrap_or_else(|error| {
    //                 panic!("Problem creating the file: {:?}", error);
    //             })
    //         } else {
    //             panic!("Problem opening the file: {:?}", error);
    //         }
    //     });
    // }

    // let _greeting_file =
    //     File::open("files/hello.txt").expect("files/hello.txt should be included in this project");

    read_username_from_file().expect("files/hello.txt should be included in this project");

    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("hello.txt");
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // let mut username = String::new();
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }
    // or
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)
    // or
    // let mut username = String::new();
    // File::open("files/hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)
    // or
    fs::read_to_string("hello.txt")
}
