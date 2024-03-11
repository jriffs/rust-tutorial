pub fn borrowing_example_1() {
    let mut s = String::from("hello"); // must be declared as mutable to have a mutable refference

    let a = &mut s; //mutable refference
    change_string(a); // a gets dropped after transferring ownership to the function

    let new_a = &s; //immutable refference can only be called because var a gets dropped 
    println!("the new string is: {}", new_a); // should print "hello everybody"
}

fn change_string(string_ref: &mut String) -> () {
    string_ref.push_str(" everybody"); // we are mutating the string thru it's reffrence
}

pub fn borrowing_example_2() {
    let s = String::from("how far");

    let a = &s;
    let b = &s; //can have any number of immutable refferences
    println!("{}, {}", a,b); 
}

pub fn dangle_example() {
    // let refference_to_nothing = dangle(); //this will not compile because the var points to nothing
    
    let refference_to_nothing = no_dangle(); //this will compile
    println!("{}",refference_to_nothing); 
}

/* fn dangle() -> &String {
    let s = String::from("a string"); // the var s gets dropped once the function gets executed
    &s // so this is only valid within the function scope
} */

fn no_dangle() -> String {
    let s = String::from("a string"); 
    s // just returning the var s is more optimal
}

pub fn print_mem_address() {
    let x = 32;
    let y = &x;
    println!("the address of the variable in mem is {:p}", y);
}

pub fn de_reff() {
    let x = 5;
    let y = &x;

    assert_eq!(5, *y); // the * derrefferences the y var to point to the data 
    println!("they are equal ..");
}

pub fn diff_type_of_reff() {
    // we can also use the "ref" keyword in place of "&", there are however some minor differences
    let chararacter = 'c';
    let rf1 = &chararacter;
    let ref rf2 =  chararacter;

    assert_eq!(*rf1, *rf2);
    assert_eq!(get_address(rf1), get_address(rf2));
    println!("{} and {} are the mem adresses", get_address(rf1), get_address(rf2));
}

fn get_address(r: &char) -> String {
    format!("{:p}", r)
}

pub fn borrowing_example_3() {
    let mut s = String::from("hello");

    let _r1 = &mut s;
    let r2 = &mut s;

    // println!("{}", r1); // this line would cause the program to panic
    println!("{}", r2); //this is valid because r2 becomes d only recognized refference cuz r1 isn't used
    println!("success ..|")
}

