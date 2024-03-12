
/* Vectors are like arrays but are dynamic (can shrink and grow) */

use std::fmt::Display;

use crate::enums::IpAddress;

pub fn vector_example_1() {
    let arr: [u8; 3] = [3, 6, 9];

    let v1 = Vec::from(arr); // creating a vector from an array

    let v2 = vec![3_u8, 6, 9]; // can use the "vec!" macro

    let v3 = vec!(3_u8, 6, 9); // can use parentheses with the vec! macro

    let v4 = vec!(arr); // this is different from the first 3

    assert_eq!(v1, v3);
}

pub fn vector_example_2() {
    let mut v1 = Vec::from([1, 3, 9]);
    v1.pop(); // [1, 3]
    v1.push(7); // [1, 3, 7]

    let mut v2 = Vec::new(); // initialized an empty vector
    v2.extend(&v1);

    assert_eq!(v1, v2);
    
    println!("success !!!");
}

pub fn vector_example_3() {
    let arr = [1, 3, 9];
    let v1 = Vec::from(arr);
    let v2: Vec<i32> = arr.into();// with the "into" method, the type to be converted to must be specified
    // the into method can also be used for String and &str types.

    assert_eq!(v1, v2);

    let v3: Vec<i32> = [1, 3, 9].into_iter().collect();//this is also like the into method but for iterables
    assert_eq!(v3, vec![1, 3, 9]);

    println!("vector_example_3 success");
}

// indexing

trait True {
    fn disp(&self) {
        println!("this types implementsthe True trait ...")
    }
}

pub fn display_exact(val: Option<&i32>) -> Box<dyn std::fmt::Display> {
        match val {
            Some(n) => Box::new(*n),
            None => Box::new("Null".to_string())
        }
    }

pub fn vector_example_4() {
    let v = Vec::from([1, 2, 3]);

    for i in 0..5 {
        // println!("{:?}", v[i]) we can't do this cuz the code will panic as the count (0..5) is out of 
        // bound for the vector (i.e the count is more than the elements in the vector)
        let index = v.get(i);
        println!("{}", display_exact(index)); // we do this instead. the method returns an Options<> type
    }
}

pub fn vector_example_5() {
    let mut v = vec![1, 2, 3, 4];

    let slice1 = &v[..];
    // let slice2 = &v[0..4]; // this code below will panic (out of bounds)
    // should use v.len() instead
    let slice2 = &v[0..v.len()];

    assert_eq!(slice1, slice2);

    // slices are read-only
    // Note: slice and &Vec are different
    let vec_ref = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..3];
    // slice3.push(4); // this isn't possible since we cannot use a slice to mutate a vec cuz slices are read-only

    assert_eq!(slice3, &[1,2,3,4]);  
}

pub fn vector_example_6() {
    // conventionally, vectors can only store elements of the same type
    // But we can work around that with the help of enums or trait objects

    // with enums
    let v1 = vec![
        IpAddress::V4("127.0.0.1".to_string()),
        IpAddress::V6("::1".to_string()),
    ];
    // now the vector has elements with 2 different variants/pseudo-types (v4 & v6)
    assert_eq!(v1[0], IpAddress::V4("127.0.0.1".to_string()));

    // with trait objects
    trait IpAddresses {
        fn disp(&self) {}
    }
    struct V4(String);
    struct V6(String);

    impl IpAddresses for V4 {
        fn disp(&self) {
            println!("{}: {}", std::any::type_name::<Self>(), self.0);
        }
    }
    impl IpAddresses for V6 {
        fn disp(&self) {
            println!("{}: {}", std::any::type_name::<Self>(), self.0);
        }
    }

    let v2: Vec<Box<dyn IpAddresses>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v2 {
        ip.disp();
    }
}



