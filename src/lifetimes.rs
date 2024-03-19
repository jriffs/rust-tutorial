use std::fmt::Debug;

pub fn lifetimes_example1() {
    // lifetimes are annotated just like generics, and you can have both on a fuction declaratioin
    // the code below is simply saying that the variables x and y must out-live the function
    // meaning that the arguments passed into the function must be heigher in scope than the function
    fn pass_x<'a, 'b, T>(x: &'a T, y: &'b T) -> &'a T {
        x
    }

    // the lifetime of the returned value must be the same as that specified
    fn pass_y<'a, 'b, T>(x: &'a mut T, y: &'b mut T) -> &'b mut T {
        y
    }
    let mut val1 = 20;
    let mut val2 = 10;
    assert_eq!(&val1, pass_x(&val1, &val2));
    assert_eq!(&10, pass_y(&mut val1, &mut val2));
}

pub fn lifetimes_example2() {
    fn bigger1<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    // using the "static" lifetime means that the value is always in scope which is the case for &str type
    fn bigger2(x: &'static str, y: &'static str) -> &'static str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let str1 = "long";
    let str2 = "longer";
    assert_eq!(bigger1(str1, str2), bigger2(str1, str2));
}

pub fn lifetimes_example3() {
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);

    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }

    let x = 10;
    let y = 20;

    let val1 = Borrowed(&x);
    let val2 = NamedBorrowed { x: &x, y: &y };
    let val3 = Either::Num(x);
    let val4 = Either::Ref(&y);

    println!("{:?}, {:?}, {:?}, {:?}", val1, val2, val3, val4);
}

pub fn lifetimes_example4() {
    #[derive(Debug)]
    struct NoCopyType;

    #[derive(Debug)]
    struct Example<'a, 'b> {
        x: &'a u32,
        y: &'b NoCopyType 
    }
    let example: Example;

    {
        let dont = NoCopyType;
        example = Example {
            x: &20,
            y: &dont
        };

        println!("{:?}", example); // this is valid cuz the 2 lifetimes are valid in this scope
    }
    // println!("{:?}", example); // this will throw an error cuz lifetime b isn't valid in this scope
}

pub fn lifetimes_example5() {
    // static lifetimes can be coereced into shorter lifetimes
    static NUM: i32 = 10; //you have to specify type when initializing constants

    fn coerced_lifetime<'a>(_: &'a str) -> &'a i32 {&NUM}

    {
        let string1: &'static str = "hello"; // the same thing without annotating the 'static
        let coerced_value = coerced_lifetime(string1);

        println!("{}", coerced_value); // now even though the value coerced_value refferences is of 
        // static lifetime, it's lifetime is no longer static but 'a 
    }
    println!("{}", NUM);
}

pub fn lifetimes_example6() {
    // using the static lifetime as a trait bound means that the type does not contain any non-static 
    // refferences or refferences to any non-static lifetime values.
    static X: i32 = 10;
    const Y: i32 = 20;
    let v = 30;

    fn print_it<T: Debug + 'static>(input: T) {
        println!("{:?}", input);
    }
    // below is the same as above, just different syntax
    fn print_it1(input: impl Debug + 'static) {
        println!("{:?}", input);
    }

    fn print_it2<T: Debug + 'static>(input: &T) {
        println!("{:?}", input);
    }

    {
        let i = 5; // owned data and contains no reffrences, this just means that the current scope "owns" the data, so in the
        // context of the current scope it is of static lifetime 

        print_it(i); // no problems here
        // print_it1(&i); // would panic
        print_it1(&X); // I think this works because X has static lifetime and a reference to X 
        // will also have Static lifetime 
        print_it1(&Y); // This is the same as above since they're both constants
        // print_it1(&v); //
        
        print_it2(&i); // no problems here
    }
}










