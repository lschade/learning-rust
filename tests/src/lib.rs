#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Rectangle) -> bool {
        return self.width == other.width && self.height == other.height;
    }
}

mod blub {
    fn private_fn() {
        println!("I am private");
    }
}


mod bla {
    use crate::blub;

    fn blub() {
        let ls = crate::Rectangle { width: 10, height: 10 };
        // blub::private_fn();
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn test_priv_fn() {
            super::blub();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::blub;

    #[test]
    fn larger_can_hold_smaller() {

        let larger = super::Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = crate::Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = super::Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = crate::Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn assert_equals() {
        println!("equals");
        let r1 = super::Rectangle {
            width: 1,
            height: 1,
        };

        let r2 = crate::Rectangle {
            width: 1,
            height: 1,
        };      

        assert_eq!(r1, r2, "Fail! {:?}", r2);
    }

    #[test]
    #[should_panic(expected="out of bounds")]
    fn should_panic() {
        println!("Test");
        let x = vec![1,2,3];
        let y = x[10];
    }
}