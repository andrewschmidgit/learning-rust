fn main() {
    from_and_into::main();
    tryfrom_and_tryinto::main();
    tf_strings::main();
}

mod from_and_into {
    use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(value: i32) -> Self {
            Number { value }
        }
    }

    pub fn main() {
        let my_str = "hello";
        let my_string = String::from(my_str);

        let num = Number::from(30);
        println!("My number is {:?}", num);

        let num: Number = 31.into();
        println!("My number is {:?}", num);
    }
}

mod tryfrom_and_tryinto {
    use std::convert::TryFrom;
    use std::convert::TryInto;

    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    pub fn main() {
        assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
        assert_eq!(EvenNumber::try_from(5), Err(()));

        let result: Result<EvenNumber, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNumber(8)));
        let result: Result<EvenNumber, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));
    }
}

mod tf_strings {
    use std::fmt;

    struct Circle {
        radius: i32,
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }

    pub fn main() {
        let circle = Circle { radius: 6 };
        println!("{}", circle.to_string());
    }
}
