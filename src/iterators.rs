use std::collections::HashMap;

pub fn iterators_example1() {
    let v = vec![1,2,3];
    for x in v {
        println!("{}", x);
    }
    // the code above and the one below are essentially the same thing
    let v = vec![1,2,3];
    for x in v.into_iter() {
        println!("{}", x);
    }
    // the for loop uses the 'into_iter()' method by default
}

pub fn iterators_example2() {
    let mut v = vec![1,2].into_iter();

    assert_eq!(v.next(), Some(1));
    assert_eq!(v.next(), Some(2));
    assert_eq!(v.next(), None);
}

pub fn iterators_example3() {
    {
        let v = vec![1,2,3];
        for i in v.iter() {
            println!("{}", i);
        }
        println!("{:?}", v); // this is possible because we used 'iter' and not 'into_iter' method
    }
    {
        let mut v = vec![1,2,3];
        if let Some(x) = v.iter_mut().next() {
            *x = 0;
        }
        assert_eq!(v, vec![0,2,3]);
    }
    {
        let mut names = vec!["jola", "ife", "ken"];
        for name in names.iter_mut() {
            *name = match name {
                &mut "ife" => "that's a dude",
                _ => "female"
            }
        }
    }
}

pub fn iterators_example4() {
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Self {
            Self { count: 1 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

pub fn iterators_example5() {
    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Fibonacci {
        fn new() -> Self {
            Self {
                curr: 0,
                next: 1
            }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            let forward = self.curr + self.next;

            self.curr = self.next;
            self.next = forward;

            Some(self.curr)
        }
    }

    let mut fib = Fibonacci::new();
    println!("{:?}", fib.curr);
    for x in 0..20 {
        println!("{:?}", fib.next());
    }
    
}

pub fn iterators_example6() {
    // there are methods that consume(use up) the iterator meaning that it cant be used after
    // here are some example

    {
        let v = vec![1,2,3];
        let v_iter = v.iter();

        let sum: i32 = v_iter.sum();
        println!("{}", sum);
    }

    {
        let names = [("sunface", 18), ("sunfei", 19)];
        let folks: HashMap<&str, i32> = names.into_iter().collect();
        let v1 = vec![1,2,3];
        let v2: Vec<i32> = v1.into_iter().collect();
        // the iteratot created in line 135 takes ownership of the v1 value due to us calling the 
        // "into_iter" method hence it can't be used angain
        println!("{:?}", folks);
        assert_eq!(v2, vec![1,2,3]);
    }

}

pub fn iterators_example7() {
    // methods that allow you to convert from one iterator to another are called iterator adaptors
    // iterator adaptors can be chained too to perform complex operations.
    let v1 = vec![1,2,3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2,3,4]);
    println!("{:?}", v2);
}








