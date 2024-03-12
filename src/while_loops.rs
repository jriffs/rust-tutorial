pub fn while_loops_example_1() {
    let mut n = 1;

    while n < 10 {
        if n % 15 == 0 {
            println!("something");
        } else if n % 5 == 0 {
            println!("something else");
        } else if n % 3 == 0 {
            println!("else");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
}

pub fn while_loops_example_2() {
    let mut counter = 1;
    // loop is used with the break and continue keywords, esle it would keep looping forever
    loop {
       counter += 1;
       if counter == 3 {
           println!("three");
           continue;
       }
       println!("{}", counter);
       if counter == 5 {
           println!("that's enough ..");
           break;
       } 
    }
    assert_eq!(counter, 5);
}


pub fn while_loops_example_3() {
    // loop is an expression, so it can be used with break to return a value
    let mut count = 1;
    let result = loop {
        count += 1;
        println!("{}",count);
        if count == 10 {
            break count * 2;
        }
    };
    assert_eq!(result, 20);
    println!("they is equal");
}

pub fn while_loops_example_4() {
    let mut count = 1;
    // you can tag/name loops for easier manipulation 
    'outer: loop {
        println!("count is: {}", count);
        'inner1: loop {
            if count >= 20 {
                break 'inner1; // this would only break inner1
            }
            count += 2;
            println!("count (inner1) becomes: {}", count);
        }

        count += 5;
        println!("count (inner2) becomes: {}", count);
        'inner2: loop {
            if count >= 30 {
                break 'outer; // this would break the outer loop
            }
            continue 'outer;
        }
    }
    assert!(count == 31);
    println!("success: {}", count);
}

