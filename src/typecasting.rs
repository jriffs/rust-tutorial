use std::fmt;
use std::str::FromStr;

/*  The *From* and *Into* traits are used to convert or "type cast" from one type to another. it basically
    is a way to define how one type can be created from another type. the 2 traits come in one package
    meaning that if you implement the From trait, the Into trait is automatically implemented as well
    Both traits don't need to be defined, they are part of the standard library
*/

pub fn typecasting_example_1() {
    // you can convert &str to String and vice versa
    let my_str = "something";

    let my_string = String::from(my_str);
    let sec_string = my_str.to_string(); // basically the same as above
    let third_string: String = my_str.into();
}

pub fn typecasting_example_2() {
    // you can implement the From trait for a custom type
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct  Number {
        value: i32
    }

    impl From<i32> for Number {
        fn from(value: i32) -> Self {
            Self {
                value
            }
        }
    }

    let number1 = Number::from(30);
    let number2: Number = 30.into();

    assert_eq!(number1, number2);
}

// TryFrom & TryInto
/*  The TryFrom and TryInto traits are like the From and Into traits with the exception being that they are
    used for fallible operations (operations that might fail), and returns a "Result" type value
*/

pub fn typecasting_example_3() {
    let n: i16 = 256;
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("This is an error when converting: {:?} but we caught it", e.to_string());
            0
        }
    };
    assert_eq!(n, 0);
    println!("typecasting_example_3 success");
}


pub fn typecasting_example_4() {

    #[derive(Debug, PartialEq)]
    struct EvenNum(i32);

    impl TryFrom<i32> for EvenNum {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNum(value))
            } else {
                Err(())
            }
        }
    }

    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    let res: Result<EvenNum, ()> = 8_i32.try_into();
    assert_eq!(res, Ok(EvenNum(8)));

    let res: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(res, Err(()));
    println!("typecasting_example_4 success");
}

/*  we can use the std::fmt::Display trait which implnts the ToString trait automatically to convert 
    custom types to String 
*/

pub fn typecasting_example_5() {
    struct Point {
        x: i32,
        y: i32
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "The point is ({}, {})", self.x, self.y)
        }
    }

    let point = Point {
        x: 0,
        y: 0
    };
    assert_eq!(point.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", point), "The point is (0, 0)");

    println!("{}", point);
}

pub fn typecasting_example_6() {
    // you can also convert a string to  and integer with the "FromStr" trait using the "from_str" method

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "5".parse::<i32>().unwrap();
    let from_str = i32::from_str("5").unwrap();
    let result = parsed + turbo_parsed + from_str;
    assert_eq!(result, 15);
    println!("typecasting_example_6 success")
}



