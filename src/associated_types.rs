trait Something {
    type ReturnType;

    fn do_something(&self) -> Self::ReturnType;
}

struct SomethingStruct {}

impl Something for SomethingStruct {
    type ReturnType = i32;

    fn do_something(&self) -> Self::ReturnType {
        return 20;
    }
}

pub fn associasted_types_example_1() {
    let somn = SomethingStruct {};
    println!("the value of the somn var is: {}", somn.do_something());
}