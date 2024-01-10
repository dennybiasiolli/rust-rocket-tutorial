pub fn add(n1: usize, n2: usize) -> usize {
    n1 + n2
}

fn _prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// these are unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expected = "Make this test fail")]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn it_returns_err() {
        let custom_err: Result<(), String> = Err(String::from("custom error"));
        assert!(custom_err.is_err());
    }

    #[test]
    fn this_test_will_pass() {
        let value = _prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = _prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn internal() {
        // we can test internal functions too
        assert_eq!(4, internal_adder(2, 2));
    }
}
