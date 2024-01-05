use std::{io, collections::HashMap};

enum Command {
    Help,
    Quit,
    Print,
    Add(String, String),
    Remove(String, String),
}

fn print_prompt_request(request_string: String) {
    println!("{request_string} [\"help\" or \"quit\" to exit]:");
}

fn print_help() {
    println!("Available commands:");
    println!("  help: print this help");
    println!("  quit: quit the program");
    println!("  add <person> to <department>: add a person to a department");
    println!("  remove <person> from <department>: remove a person to a department");
}

fn parse_command(input: String) -> Option<Command> {
    match input.to_lowercase().trim() {
        "h" | "help" => Some(Command::Help),
        "q" | "quit" => Some(Command::Quit),
        "p" | "print" => Some(Command::Print),
        _ => {
            let mut words = input.split_whitespace();
            let command = words.next().unwrap_or("");
            let person = words.next().unwrap_or("");
            let _to_from = words.next().unwrap_or("");
            let department = words.next().unwrap_or("");
            if command=="add" && _to_from=="to" {
                Some(Command::Add(person.to_string(), department.to_string()))
            } else if command=="remove" && _to_from=="from" {
                Some(Command::Remove(person.to_string(), department.to_string()))
            } else {
                None
            }
        },
    }
}

fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    print_prompt_request(String::from("Type your command"));
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        match parse_command(command) {
            Some(Command::Help) => print_help(),
            Some(Command::Quit) => break,
            Some(Command::Print) => println!("{:?}", map),
            Some(Command::Add(person, department)) => {
                println!("Adding {} to {}", person, department);
                let persons = map.entry(department.clone()).or_insert(vec![]);
                if !persons.contains(&person) {
                    persons.push(person);
                } else {
                    println!("{} already in {}", person, department);
                }
                println!("{:?}", map);
            },
            Some(Command::Remove(person, department)) => {
                println!("Removing {} from {}", person, department);
                let persons = map.entry(department.clone()).or_insert(vec![]);
                if persons.contains(&person) {
                    persons.retain(|x| x != &person);
                } else {
                    println!("{} not in {}", person, department);
                }
                println!("{:?}", map);
            },
            _ => print_prompt_request(String::from("Wrong command")),
        }
    }
}
