// a match is an expression so it can be used in assingment
// it can also be used to show the variants in enums
use crate::enums::Message;
pub fn match_example_1() {
    let messages: [Message; 3] = [
        Message::Quit,
        Message::Move {
            x: String::from("some string"),
            y: 10,
        },
        Message::ChangeColor(255, 255, 0),
    ];

    for message in messages {
        show_message(message);
    }

    fn show_message(m: Message) {
        match m {
            Message::Move { x: a, y: b } => {
                assert!(b > 5);
            }
            Message::ChangeColor(r, g, b) => {
                assert_eq!(r, 255);
            }
            Message::Write(s) => {
                println!("Nothing to see here");
            }
            _ => {
                // this takes care of every other variant of option, esp if you do not care about it
                println!("No more values to check");
            }
        }
    }
}

pub fn match_example_2() {
    // the matches keyword is used to check if a variable matches a pattern
    let arr = ['a', 'Z', '9', 'L', 'b', 'a'];

    for c in arr {
        assert!(matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9'));
    }

    println!("success");
}

pub fn match_example_3() {
    enum Somn {
        Foo,
        Bar,
    }
    let mut count = 0;
    let v = vec![Somn::Foo, Somn::Bar, Somn::Foo];
    for vector in v {
        if matches!(vector, Somn::Foo) {
            count += 1;
        }
    }
    println!("{}", count);
}

pub fn match_example_4() {
    // sometimes u can use if let instead of match if the case is small
    // especially when destructuring an Option type variable
    let vab = Some(30);
    // instead of a match statement, use if let
    if let Some(i) = vab {
        println!("the variable value is: {}", i);
    }
}

pub fn match_example_5() {
    // the @ keyword is used to create a variable and match it's value to a predefined set at the same time
    enum Messages {
        Hello { id: i32 },
    }

    let message = Messages::Hello { id: 10 };
    match message {
        Messages::Hello { id: a @ 3..=7 } => println!("id in range [3..7] is: {}", a),
        Messages::Hello {
            id: b @ (10 | 11 | 12),
        } => println!("id in another range [10,15,20]: {}", b),
        _ => println!("something is happening"),
    }
}

pub fn match_example_6() {
    // if gaurd is an if condition put after a match variant that must match for that variant to be chosen
    let num = Some(5);
    match num {
        Some(x) if x < 10 => println!("x is less than 10"),
        _ => println!("nothing to show"),
    }
}

pub fn match_example_7() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
    match numbers {
        (first, .., last) => println!("first and last are: {} & {}", first, last),
        _ => println!("nothing more to see")
    }
}

pub fn match_example_8() {
    let mut v = String::from("Hello");
    let r = &mut v;

    match r {
        value => value.push_str(" world")
    }
    println!("{}", v);
}

