pub fn if_statements_example_1() {
    // you can use if statements for variable assingments
    let n = 5; let m = 10;
    let assigned = 
        if n > 0 && m <= 10 {
            println!("it is in range");
            m * 2
        } else if n == 0 {
            println!("it is zero");
            n
        } else {
            0
        };

    println!("{}", assigned);
}


pub fn if_statements_example_2() {
    //  the use of the ".iter().enumerate" method
    let arr = ['a','b','c','d','e'];

    for (i,v) in arr.iter().enumerate() {
        println!("the {}th element is {}", i+1,v);
    }
    /* so above works like the regular for loop in JS where i would be the iterator and v would be the
        value.
     */ 
}


