use std::fmt;
//

pub fn display_trait_exmple1() {
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;

            for (count, val) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", count, val)?;
            }

            write!(f, "]")
        }
    }

    let vector = List(vec![1,2,3]);
    println!("{}", vector);
}



