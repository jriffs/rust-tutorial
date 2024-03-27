use std::{fmt::Debug, ops::Add};

pub fn closures_example_1() {
    // Let's say we were to increase a value or variable by one, That would be too simple to use a 
    // function for So instead, you could use an inline closure. It should be simple and straight to 
    // the point. 

    fn add_one(val: i32) -> i32 {
        val + 1
    }
    // the above is too tedious to achieve something so little, instead we use a closure
    let closure_annotated = |val: i32| -> i32 {val + 1}; // the types are annotated
    let closure_inffered = |val| val + 1; // the types are inffered by 
    // the compiler when the closure is called

    let return_1 = || 1; // this closure simply just returns the value 1

    assert_eq!(closure_inffered(3), 4);
    println!("success ..")
}

pub fn closures_example_2() {
    // capturing variables in closures

    let hello = String::from("hello");

    // let print = move || println!("{}", hello); // 'move' here means the closure takes ownership of hello, 
    // which would make the code at line 34 result in an error
    let print = || println!("{}", hello);

    print();
    print();

    let _reborrow = &hello;
    println!("{}", hello);
}

pub fn closures_example_3() {
    let mut count = 0;

    // So the code below will give an error basically because The variable count is being borrowed as 
    // mutable which is a problem because we also create an immutable refference in line 51, and we know
    // we can't have a mutable and immutable refference at the same time  
    /* let mut increase = || {
        count += 1;
        println!("{}", count);
    }; */
    // the code below fixes the problem above by introducing the move keyword. since count is of type i32
    // it implements the copy trait, so it will basically get copied into the closure instead being 
    // outrightly owned by it, so now we can have an immutable refferene for count
    let mut increase = move || {
        count += 1;
        println!("{}", count);
    };

    increase();

    let re_borrowed = &count;

    increase();

    let count_reborroed = &mut count;
    // the line above only works because we do not use the 're_borrowed' variable again after declaring
    println!("{}", count_reborroed); 
}

pub fn closures_example_4() {
    let movable = Box::new(10);
    // this closure below implements the 'FnOnce' function trait because there is a change of ownership 
    // of the captured variable 'movable'. 
    /* let consume = || {
        println!("{}", movable);
        take(movable);
    }; */
    // to fix this issue we simply have to change the 'take' function to take a reffernce to 'movable
    let consume = || {
        println!("{}", movable);
        take(&movable);
    };

    consume();
    consume();

    fn take<T>(var: &T) {}
}

pub fn closures_example_5() {
    // when using closures as function parameters, the entire signature of the closure has to be 
    // defined/annotated

    fn fn_once<F>(func: F)
    where F: Fn(usize) -> bool 
    {
        println!("{}", func(3));
        println!("{}", func(4));
    }

    let closure = |val: usize| -> bool {
        if val > 5 {
            true
        } else {
            false
        }
    };
    fn_once(closure);
    // the method above would be considered verbose and unnessecarily tedious
    // here's a beter way
    let closure = |val: usize| -> bool {val > 2};
    fn_once(closure);
    // the above code is less tedious but still not the best, lets optimize it below

    {
        let vec = vec![1,2,3];
        fn_once(|val| val == vec.len());
    }
}

pub fn closures_example_6() {
    // the compiler uses the most non-restrictive approach when it comes to closures.
    // Fn b4 FnMut b4 FnOnce

    fn apply<F>(f: F)
    where F: FnOnce() {
        f();
    }

    fn apply_to_3<F>(f: F) -> i32
    where F: Fn(i32) -> i32 {
        f(3)
    }

    {
        use std::mem;
        
        let greeting = "hello";
        // the 'to+owned' method is used to create owned data from borrowed data 
        let mut farewell = "goodbye".to_owned();

        let salute = || {
            // the non restrictive approach
            println!("{}", greeting);
            // at this point the compiler maintains the 'Fn' trait cuz the variable doesn't nessecarily
            // need to be moved for a print statement, meaning the variable is captured by refference

            // the lesser restrictive approach
            farewell.push_str("!!!");
            println!("{}", farewell);
            // the code above forces the variable to be captured as a mutable refference
            // thus causing the compiler to infer the FnMut trait

            mem::drop(farewell);
            // the above code requires that the closure take ownership of the farewell variable
            // thus causing the compiler to infer the FnOnce trait for the closure
            // the basic idea is that the trait inferered depends on the action performed on the captured
            // variables

        };

        println!("3 doubled is: {}", apply_to_3(|x| x * 2)); // prints out 6

        apply(salute);
    }
    
}

pub fn closures_example_7() {
    // the move keyword basically only specifies how the values are captured

    fn exec<F>(f: F) where F: FnOnce() {
        f();
    }

    fn exec_2<F>(f: F) where F: Fn() {
        f();
    }
    let s1 = String::from("hello");
    let s2 = String::from("bye !!");
    let closure1 = move || println!("{}", s1);
    let closure2 = move || println!("{}", s2);

    exec(closure1);
    exec_2(closure2);
    // the above code has no errors even if the exec function specifies the 'FnOnce' trait but gets
    // the 'Fn' trait
}

pub fn closures_example_8() {
    let mut s = String::new();
    let update_string = |str| -> String {s.push_str(str); s};
    // the compiler immediately infers 'FnOnce' when we try to return 's'
    // this is because you have to own a value to return it.

    fn exec<'a, F>(f: F) where F: FnOnce(&'a str) -> String {
        f("hello");
    }

    exec(update_string);
}

pub fn closures_example_9() {
    // we can also use functions as parameters, just like closures

    fn display<F: Fn()>(f: F) {
        f();
    }

    let closure = || println!("i'm a closure.");

    fn function() {println!("i'm a function.")}

    display(closure);
    display(function);
}

pub fn closures_example_10() {
    // using closures as return types
    // using static dispatch
    fn create_fn1() -> impl Fn(i32) -> i32 {
        let num = 5;
        move |x| x + num 
        // the 'move' keyword is used here because we need to take ownership of the num value to return it 
        // this is so it does'nt go out of scope after we call the function 
    }
    // using dynamic dispatch
    fn create_fn2() -> Box<dyn Fn(i32) -> i32> {
        let num = 5;
        Box::new(move |x| x + num ) 
    }

    /* let get_function = create_fn1(); */ // this is the same as bellow
    let get_function = create_fn2();
    println!("{}", get_function(1));
}

pub fn closures_example_11() {
    fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
        let num = 5;

        if x > 1 {
            Box::new(move |x| x + num)
        } else {
            Box::new(move |x| x + num)
        }
    }

    // in the code above, even though the 2 closures are identical, they still hold 2 diffrent locations 
    // in memory, and the compiler does not know which to return at compile time, 
    // hence the dynamic dispatch
}










