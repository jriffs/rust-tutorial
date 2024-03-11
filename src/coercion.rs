


/*  Types conversion (also knwn as coercion) is basically the changing or conversion of primitive types
    from one to another.
*/

pub fn coercion_example_1() {
    let decimal = 97.123_f32;

    let integer = decimal as u8;

    let c1 = decimal as u8 as char; // the as keyword can be chained
    let c2 = integer as char;

    assert_eq!(integer + 1, 'b' as u8);
    println!("coercion_example_1() success");
}

#[allow(overflowing_literals)]
pub fn coercion_example_2() {
    assert_eq!(u8::MAX, 255);
    let v = 1000 as u8;

    println!("v is {}", v); // 232
    /*  Basically what happens here is that the compiler deducts T::MAX + 1 (where T is the primitive type) 
        until the value fits into the range of the type it is converting to. in this scenario:
        u8::MAX + 1 = 256, then
        1000 - 256 = 744 > 255 so repeat
        744 - 256 = 488 > 255 so repeat
        488 - 256 = 232 <= 256
        so 232 is returned 
    */
}

#[allow(overflowing_literals)]
pub fn coercion_example_3() {
    assert_eq!(-1_i8 as u8, 255); // it basically just rotates returning the highest possible value (255)
    
    /*  Since Rust 1.45 the 'as' keyword does a "saturating cast" when converting from float to int
        meaning that if the number falls bellow of above the upper or lower bounds of the type, it would
        return the bound that was crossed
    */
    assert_eq!(300.1_f32 as u8, 255); // this would return 255 becuz 255 is the bound that was crossed (T::MAX)
    assert_eq!(-100.1_f32 as u8, 0); // this would return 0 becuz 0 is the bound that was crossed (T::MIN)

    /*  The following codes behaviour incurs a small runtime cost but can be avoided by using with 
        "Unsafe methods" however the result might overflow and may return 'unsound values
    */

    unsafe {
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // u8::MAX + 1 = 256, 256 - 100 = 156 
        println!("-100.0 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("NaN is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}

