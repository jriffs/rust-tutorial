use std::i32;

pub enum Message {
    Quit,
    Move{x: String, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum IpAddress {
    V4(String),
    V6(String)
}

pub fn enums_example_1() {
    // enums here are like in JS/TS
    // you can't use floating points to instantiate an enum
    let home_ip = IpAddress::V4(String::from("127.0.0.1")); // this is how to use an enum
}

pub fn enums_example_2() {
    enum Number {
        Zero, // is implicitly initialized with the value 0
        One, // they always follow an ascending order, so this would be initialized with the value 1
        Two // and this 2
    }

    enum Number2 {
        Zero = 0,
        One = 1,
        Two = 2
    }

    enum Number3 {
        Five = 5, // you can start from any number
        One, // but the compiler infers in ascending order, so this would be initialised with 6
        Two // and this 7
    }

    assert_eq!(Number::One as u8, Number2::One as u8); // they need to be converted to integers to be 
    assert_eq!(Number::Two as u8, Number2::Two as u8); // used in the assert_eq! macro

    println!("success");
}

pub fn enums_example_3() {
    let msg1 = Message::Move { x: String::from("hello"), y: 10 };
    let msg2 = Message::Write("some string".to_string());

    println!("success");    
}

//Option enum
/*  the option enum is an enum having only 2 options, characterized by its definition in the std lib as:
    enum Option<T> {
        None,
        Some(T)
    }
    it is used when there is a chance that a function or expression might not return a value
    it is similar to the null operator in other languages like JS
*/ 
pub fn enums_example_4() {
    let five = Some(5);
    let none: Option<i32> = Option::None; //don't need to add the "Option" head
    let nothing = plus_one(None);
    let something = plus_one(five);

    if let Some(n) = something { // this line detructures the Some(i32) variant
        println!("{}", n);
    } else {
        panic!("Something went wrong !!!");
    }

    fn plus_one(p: Option<i32>) -> Option<i32> {
        // the match statement return the result of comparing the value of p
        match p {
            None => None,
            Some(i) => Some(i + 1)
        }
    }
}


