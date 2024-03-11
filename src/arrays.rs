#[allow(unused)]
use std::{mem, ops};

pub fn array_signature_example() {
    let arr: [char; 5] = ['n', 'e', 'i', 'm', 's'];
    println!("the size of the array is: {}bytes", mem::size_of_val(&arr));
}

pub fn array_example_1() {
    let arr = [1; 50]; // this declares all the elements in the array as whatever we declare (1)
    assert!(arr[0] == 1);
    assert!(arr.len() == 50);
    println!("success");
}

pub fn array_example_2() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[0..3]; // this is how to get a slice, they are usually annotated as a reff
    assert_eq!(slice, &[1, 2, 3]);
    println!("success");
}

pub fn array_example_3() {
    let arr = ['e', 'f', 'g', 'h', 'i'];

    let slice = &arr[..3]; // if it starts ar zero, you can ommit the zero

    assert!(std::mem::size_of_val(&slice) == 16);
    /*  the ref to the slice variable is not an array, it is just a pointer object that points to 
        a group of values in an array, it has 2 parts: the main pointer to the values in mem, and the length.
        because of this, the size of those 2 components is 2 * (usize)
        which on a 64bit system would be 8bytes
     */
    println!("the size of the slice variable is: {}bytes", std::mem::size_of_val(&slice));
}

