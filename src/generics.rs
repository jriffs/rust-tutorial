#[derive(Debug)]
struct A; // unit struct, can have implementations but no fields.
struct S(A); // tuple struct
struct SGen<T>(T); // that is how a generic is specified for a struct (after the struct name)

fn reg_fn(_s: S) {} //this is a regular function expression
fn gen_spec_t(_s: SGen<A>) {} // this function takes a param "s" of type/struct "SGen"
fn gen_spec_i32(_s: SGen<i32>) {} // this function takes a param "s" of type/struct "SGen"
fn generic<T>(_s: SGen<T>) {} // this is a generic function expression

pub fn generics_example_1() {
    reg_fn(S(A)); // called normally
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(30));

    //explicitly define the type
    generic::<char>(SGen('A'));
    //implicitly define the type
    generic(SGen(String::from("hello")));
    println!("success ..")
}

pub fn generics_example_2() {
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 10, y: 20 };
    let float = Point { x: 1.8, y: 10.6 };
    println!("evevrything set");
}
pub fn generics_example_3() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let mixed = Point {
        x: 10,
        y: "something",
    };
    println!("evevrything set");
}

pub fn generics_example_4() {
    struct Val<T> {
        val: T,
    }

    impl<T> Val<T> {
        fn value(&self) -> &T {
            &self.val
        }
    }
    let x = Val { val: 3.0 };
    let y = Val { val: String::from("something") };
    println!("x.val is: {:?}, y.val is: {:?}", x.value(), y.value());
}

pub fn generics_example_5() {
    struct Pointers<T, U> {
        x: T,
        y: U
    }

    impl<T, U> Pointers<T, U> {
        // self is used instead of "&self" cuz we do not care about ownership as we are not using p1 after
        // it is called in line 80.
       fn mixup<P, Q>(self, sec: Pointers<P, Q>) -> Pointers<T, Q> {
         Pointers {
            x: self.x,
            y: sec.y
         }
       } 
    }

    let p1 = Pointers{x: 5, y: 10};
    let p2 = Pointers{x: "Hello", y: 'P'};
    let p3 = p1.mixup(p2);
    

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, 'P');
    println!("e go well ..");
}

/* you can expressly specify a concrete ype for an impl block
    impl SomeStruct<i32> {
        fn do_something() {} ...
    }
*/

