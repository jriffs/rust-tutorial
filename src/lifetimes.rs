pub fn lifetimes_example1() {
    // lifetimes are annotated just like generics, and you can have both on a fuction declaratioin
    // the code below is simply saying that the variables x and y must outlive the function
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










