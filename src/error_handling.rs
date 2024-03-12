use std::num::ParseIntError;
use std::fs::File;
use std::io::{self, Read};



pub fn error_handling_example_1() {
    // the simplest form of error handling is with the use of the "panic!()" macro
    let mut num = 7u8;
    if num % 2 == 0 {
        num += 2;
        println!(" we added to num and it is now: {}", num);
    } else {
        panic!()
    }
    println!("this line won't run when the program panics");
}

pub fn error_handling_example_2() {
    fn multpl_str(x: &str, y: &str) -> Result<i32, ParseIntError> {
        let num1 = x.parse::<i32>();
        let num2 = y.parse::<i32>();
        Ok(num1.unwrap() * num2.unwrap())
    }

    let result = multpl_str("10", "2");
    

    let result = multpl_str("t", "2");
    // The line above will throw an error
}


pub fn error_handling_example_3() {
    // The "?" kkeyword works like "unwrap", but returns instead of panic
    fn multpl_str(x: &str, y: &str) -> Result<i32, ParseIntError> {
        let num1 = x.parse::<i32>()?;
        let num2 = y.parse::<i32>()?;
        Ok(num1 * num2)
    }

    assert_eq!(multpl_str("10", "2").unwrap(), 20);

    assert_eq!(multpl_str("j", "2").unwrap(), 20); // this will cause an error
    println!("error_handling_example_3");
}


pub fn error_handling_example_4() {
    fn read_file1() -> Result<String, io::Error> {
        let f = File::open("src/hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e)
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e)
        }
    }

    fn read_file2() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("src/hello.txt")?.read_to_string(&mut s)?;
        // the 2 "?" operators basically replace the match statements in read_file1() function above

        Ok(s)
    }

    assert_eq!(read_file1().unwrap(), read_file2().unwrap());
    let res = read_file1().unwrap();
    println!("{}", res);
}

pub fn error_handling_example_5() {
    // using 'map' and 'and_then'
    fn add_two_1(val: &str) -> Result<i32, ParseIntError> {
        val.parse::<i32>().map(|n| n + 2)
    }

    fn add_two_2(val: &str) -> Result<i32, ParseIntError> {
        val.parse::<i32>().and_then(|n| Ok(n + 2))
    }

    assert_eq!(add_two_1("4").unwrap(), 6);
    assert_eq!(add_two_1("4").unwrap(), add_two_2("4").unwrap());
}

pub fn error_handling_example_6() {
    // this would be the verbose way to achieve the desired result
    fn multpl_str_again1(str1: &str, str2: &str) -> Result<i32, ParseIntError> {
        match str1.parse::<i32>() {
            Ok(n1) => {
                match str2.parse::<i32>() {
                    Ok(n2) => Ok(n1 * n2),
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }

    // this would be the more succint way of doing the same as the function above
    fn multpl_str_again2(str1: &str, str2: &str) -> Result<i32, ParseIntError> {
        str1.parse::<i32>()
        .and_then(|n1| str2.parse::<i32>().map(|n2| n1 * n2))
    }

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e)
        }
    }

    let num1 = multpl_str_again2("10", "2");
    print(num1);
    let num2 = multpl_str_again2("tt", "2");
    print(num2);
}


// Using type aliases instead of the original types can help make ur code less verbose
type Res = Result<i32, ParseIntError>;
pub fn error_handling_example_7() {
    fn multpl_str_again1(str1: &str, str2: &str) -> Res {
        match str1.parse::<i32>() {
            Ok(n1) => {
                match str2.parse::<i32>() {
                    Ok(n2) => Ok(n1 * n2),
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }

    // this would be the more succint way of doing the same as the function above
    fn multpl_str_again2(str1: &str, str2: &str) -> Res {
        str1.parse::<i32>()
        .and_then(|n1| str2.parse::<i32>().map(|n2| n1 * n2))
    }

    fn print(result: Res) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e)
        }
    }

    print(multpl_str_again2("10", "2"));
    print(multpl_str_again2("tt", "2"));
}







