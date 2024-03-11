pub fn ownership_example_1() {
    let x = String::from("what's up world"); // this is how to create a string value
    // string values are considered dynamic therefore will be stored in heap memory, hence the variable x 
    // is principally just a pointer to the allocated heap memory for the string value.
    // let y = x; // doing this will make the memory allocated for variable x to be cleared
    let y = x.clone(); // this however create a deep copy of the pointer and creates a new value 
    // in the heap memory where the pointer points to so x is no longer cleared from memory.
    println!("{}, {}", x,y);
}

pub fn ownership_example_2() {
    let s1 = String::from("hello");
    let s2 = take_ownership(s1);

    // println!("{}", s1); // this would result in a panic because the ownership of s1 has been moved to 
    // the "take_ownership" function.
    println!("{} again", s2);
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

pub fn mutability_change_example() {
    let x = String::from("hello");

    let mut y = x; // when x gets dropped and it's value assinged to y, we can assing y as mutable

    y.push_str(" world");
}

pub fn heap_reff_example() {
    let x = Box::new(5); // Box allocates values directly to heap memory and x becomes a pointer
    let mut y = Box::new(1);
    // * is used to refference the value in heap memory as y is only a pointer;
    *y = 4; // this actualy drops the innitail val in heap mem, assings a new val in memory and mutates
    // the the pointer y to point to the new heap mem value

    assert_eq!(*x, 5);
    println!("*x should give {}", *x);
}

pub fn partial_ownership_example() {
    // a struct is like an object in JS
    struct Person {
        name: String,
        age: Box<u8>
    }
    // the person1 variable is initialised as a person struct
    let person1 = Person {
        name: String::from("john"),
        age: Box::new(20)
    };
    /* Below is a destructuring assingment. name is a variable separate from person1.name as it was 
    destructured directly, but age is just a refference to person.age because the ref keyword was used. 
    now person1 no longer has ownership of the name attribute so person.age will panic, also 
    person1 cannot be used as a whole variable anymore: println!("{}", person) would panic*/
    let Person {name, ref age} = person1;
    println!("the name variable is {}", name);
    println!("the age variable is {}", age);

    // println!("person1 is {:?}", person1); // this would cause code to panic
    println!("person1.age is {:?}", person1.age)// but this is okay
}

pub fn dest_tuple_example1() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0; // this is used to refference items in the tuple.
    // the value of t.0 here has been partially moved just like before, so only t.1 can be used

    println!("t now only has one item: {}", t.1);
}

pub fn dest_tuple_example2() {
    let t = (String::from("hello"), String::from("world"));

    let (ref x, ref y) = t;
    println!("x, y and t are: {:?}, {:?} & {:?} ", x, y, t); // this is only possible because we refferenced t.0 & t.1
    // we could've also done this: let (x, y) = t.clone();
}

