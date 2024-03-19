#![allow(unused, dead_code)] // this line removes the compiler warning. there are many other options
// variables are declared with the let keyword, but unlike javascript, 
// the let in Rust behaves like the "const" in javascript.

use std::ops::{Range, RangeInclusive}; // imports the ops module from the standard library
use std::mem::size_of_val;
mod test_module;
mod ownership_exercises;
mod borrowing;
mod arrays;
mod tuple;
mod struct_example;
mod enums;
mod if_statements;
mod while_loops;
mod match_example;
mod methods;
mod generics;
mod traits;
mod associated_types;
mod vectors;
mod hashmaps;
mod coercion;
mod typecasting;
mod error_handling;
mod display_trait;
mod lifetimes;
fn main() {
    // let x: i32; // using uninitialized variables will result in an error 
    let x: i32 = 5;
    // let y: i32; // unused uninitialized variables will only cause a warning
    let _y: i32; // adding an underscore to the variable will remove the warning

    assert_eq!(x, 5);   // this function is a core macro (like in-built API functions in javascript)
    // it checks if 2 entities are equal.
    println!("{}", x);  /* printing a variable to the console/output. the "{}" expression tells rustc
    to treat whatever is in the parentheses as a string, basically converting the variable to a 
    string literal */        
    println!("success ...");
    lifetimes::lifetimes_example6();
}

// functions are declared using the "fn" keyword.
fn testing_function_declaration() { // function names are declared in snake case (with underscore)
    println!("testing this function ..");
}

fn mutable_example() {
    let mut x = 1; // the "mut" keyword means that the variable can be mutated/changed
    x += 2; // this is only possible cuz of the "mut keyword"

    assert_eq!(x, 3);
    println!("it's equal");
}

fn shadowing_example() {
    let x = 5; // in the main scope, x is declared as 5
    println!("variable x is = {} at first", x); // should print 5

    {
        let x = 12;
        assert_eq!(x, 12); // this will run successfully, cuz the x in this scope is not the same as in the main scope
    }

    let x = 42; // this is shadowing, basically re-Initializing x with a different value
    println!("variable x is = {} later", x); // should print 42
}

fn destructuring_example() {
    // first method
    let (mut x, y) = (1, 3); // the RHS is a tuple. a tuple can house any type of value
    x += 2; // this is possible cuz of the "mut" keyword up there
    assert_eq!(x, y);

    // second method
    let (a, b);
    (a, ..) = (10, 12);
    [.., b] = [3, 8];
    assert_eq!([a, b], [10, 8]);
    println!("should run smoothly");
}

fn as_example() {
    let x: i32 = 38_u8 as i32; /* you can directly specify an integers type as seen by the "_u8" 
    appended to the value 38. 
    the "as" keyword is also used to convert one integer type to another.
    */
    let y = 38;
    // x = y; // this will throw an erro, as you cannot assign a value of a type to another type
    // x = y as i32; // this also will not work
    assert_eq!(x, y, "this is a panic message, and shouldn't show if the assertion is true");
    println!("as example good.");
}

fn main_function() {
    println!("this is the main house");
    let x = sub_function();
    println!("{}", x);

    fn sub_function() -> String {
        // return "this is the sub function."; // this returns a value from a function
        "this is the sub function.".to_string() // this does the same thing above.
    }
}

fn type_infer_example() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }
    println!("function successful");
}

fn max_integers() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("the max values for i8 and u8 integers are {} and {}", i8::MAX, u8::MAX);
}

fn using_diff_numbers() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("success");
}

fn floating_point_precision() {
    // assert!(0.1 + 0.2 == 0.3); // this will panic cuz the values are in the default floting 
    // point type (f64) which is very precise, meaning 0.1 + 0.2 might give 0.30000000001
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32); // this fixes the problem as f32 is less precise
    println!("all equal ..");
}

fn for_loops_example() {
    let mut x = 0;
    for i in -3..2 {
        x += i
    }
    assert!(x == -5);

    for c in 'a'..'z' {
        println!("{}", c);
    }
}

fn range_example() {
    assert_eq!((1..5), Range{start: 1, end: 5});
    assert_eq!((1..=5), RangeInclusive::new(1,5));
    println!("range success");
}

fn bitwse_operations_example() {
    let c = 1<<5;
    println!("{}", c);
    println!("0101 AND 0011 is {:04b}", 0101 & 0011);
}

fn char_example() {
    let c = 'a';
    println!("{}", size_of_val(&c)); // should print 4 which is the size of all unicode characters
}

fn destructuring_tuple_struct() {
    // the code below is how your would destructure a tupke struct
    struct Point(i32, i32);

    let point = Point(3, 5);

    // Destructuring the tuple struct
    let Point(x, y) = point;

    println!("x: {}, y: {}", x, y);
}



