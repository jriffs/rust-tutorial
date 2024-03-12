pub fn tuple_example_1() {
    let tuple = (32_u32, -10, "some string", String::from("hello"));
    let first = checked_type(&tuple.0);
    println!("the type of the tuple element is: {}", first);
    
    fn checked_type<T>(val: T) -> String {
        format!("{}", std::any::type_name::<T>())
    }
}
