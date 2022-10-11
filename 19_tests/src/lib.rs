// As usual with a library, start with some functional code that needs testing.
// In this case it's a struct representing a rectangle with a method telling
// us if another rectangle fits inside it.
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// the tests always belong in a module decorated with `#[cfg(test)]`
#[cfg(test)]
mod tests {
    // they need to start with this use statement
    use super::*;

    // each individual test needs to be decorated with `#[test]`
    // the name of the function is the name of the test if you
    // need to reference it on the command line or test report
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // the assert statement needs to hold code that returns a boolean
        // if the boolean is true the test passes and vice versa
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // if we want a condition to return false we can negate the boolean
        // with a `!` prefix to tell the compiler that false is correct
        assert!(!smaller.can_hold(&larger));
    }

    // this is an example of a failing test
    #[test]
    fn failing_test() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        // asking if two variable equal each other is such a common assertion
        // that we can use this shortcut function
        assert_eq!(4, add_two(2));

        // there is also an equivalent not equal assertion
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn custom_fail_message() {
        // we can add custom messages to assertions to make a failure message easier to 
        // understand. This includes adding the result of the function to the message.
        let result = add_two(2);
        assert!(
            result == 5,
            "2 + 2 should equal 4, instead it equalled `{}`",
            result
        );
    }

    // some tests take a really long time to run and while important to check
    // now and again we don't want to run them all the time. So we can add the 
    // ignore flag to them meaning they won't be run when you run `cargo test`.
    // to include then run `cargo test -- --include-ignored`.
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
