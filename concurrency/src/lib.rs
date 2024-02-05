pub mod threads {
    use std::thread;
    use std::time::Duration;

    pub fn test1() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        // without this, when the main thread ends, the spawned
        // thread will be killed
        handle.join().unwrap();
    }

    pub fn test2() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }
}

pub mod channels {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    fn create_thread(tx: mpsc::Sender<String>, vals: Vec<String>) -> thread::JoinHandle<()> {
        return thread::spawn(move || {
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(200));
            }
        })
    }

    pub fn test1() {
        let (tx, rx) = mpsc::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        create_thread(tx.clone(), vals);

        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        create_thread(tx, vals);

        let handle = thread::spawn(move || {
            for received in rx {
                println!("Got: {}", received);
            }
        });
        handle.join().unwrap();
    }
}
