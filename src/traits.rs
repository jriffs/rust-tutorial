use std::ops;

trait Hello {
    // this is a default method, you dont need to implement it for the types
    fn say_hello(&self) -> String {
        String::from("hello")
    }
    fn say_something(&self) -> String; // this needs to be implemented
}
#[derive(PartialEq, PartialOrd)] // this line makes it possible to use derived traits
struct Student {}

impl Hello for Student {
    fn say_something(&self) -> String {
        String::from("i'm a good student")
    }
}

struct Teacher {}

impl Hello for Teacher {
    // you can redeclare the default function even if you don't need to, but you can
    fn say_hello(&self) -> String {
        String::from("Hi i'm your new teacher")
    }
    fn say_something(&self) -> String {
        String::from("i'm not a bad teacher")
    }
}

pub fn traits_example_1() {
    let student = Student{};
    let teacher = Teacher{};

    assert_eq!(student.say_hello(), "hello");
    assert_eq!(student.say_something(), "i'm a good student");
    assert_eq!(teacher.say_hello(), "Hi i'm your new teacher");
    assert_eq!(teacher.say_something(), "i'm not a bad teacher");
    println!("success ..");
}

pub fn traits_example_2() {
    // this function needs the generic var T to implement the Mul trait
    fn multiply<T: ops::Mul<Output = T>>(a: T, b: T) -> T {
        a * b
    }

    println!("{}", multiply(10, 21));
}

trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("the author of post {}, is {}", self.title, self.author)
    }
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}

fn summary<T: Summary>(s: &T) { //first way to do it
    let output = s.summarize();
    println!("{:?}", output);
}

fn another_summary(s: &impl Summary) { //second way to do it
    let output = s.summarize();
    println!("{:?}", output);
}
pub fn traits_example_3() {
    let post1 = Post{
        title: "what's going on".to_string(),
        author: "jedi of javascript".to_string(),
        content: "something is up with my emulator".to_string()
    };
    let weibo1 = Weibo{
        username: "chaampion11265653".to_string(),
        content: "i love rust so much".to_string()
    };

    summary(&post1);
    another_summary(&weibo1);
}

trait Building {
    fn has_stairs(&self) -> bool;
}

struct StoryBuilding {}

struct Bungalllow {}

impl Building for StoryBuilding {
    fn has_stairs(&self) -> bool {
        true
    }
}

impl Building for Bungalllow {
    fn has_stairs(&self) -> bool {
        false
    }
}

/* So you can't use 'impl Trait' to specify the return type of a function if the function returns more 
than one type, you have to use a trait object.
Rust has 2 ways of handling traits and its implementations: 
1. Static Dispatch: this means that the compiler knows tha methods that each type has and creates functions 
for each
2. Dynamic Dispatch: this means that the methods are only known at runtime. this is done by specifying the
&dyn Trait or using the "Box" keyword by doing this: 'Box<dyn Trait>'. what this does is create a vtable
which houses all the types and the methods they have, and then returns a pointer to the type in the vtable,
then the program 
although this is not as efficient as the dynamic dispatch because of the overhead of looking for the 
appropriate method in the vtable    

*/
pub fn traits_example_4() {
    fn construct_house(no_of_stairs: i32) -> Box<dyn Building> { //specifies that it returns a dynamic type
        if no_of_stairs >= 1 {
            Box::new(StoryBuilding {})
        } else {
            Box::new(Bungalllow {})
        }
    }

    let house = construct_house(2);
    println!("does the house have stairs: {}", house.has_stairs());
}

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }
}

impl<T: std::fmt::Debug + PartialOrd + PartialEq> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest number is x = {:?}", self.x)
        } else {
            println!("the largest number is y = {:?}", self.y)
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Unit(i32);

pub fn traits_example_5() {
    let pair1 = Pair::new(Unit(1), Unit(2));
    pair1.cmp_display();
}

trait Bird {
    fn quack(&self) -> String;
}

struct Duck;

struct Swan;

impl Duck {
    fn swim(&self) {
        println!("look the duck is swimming ...");
    }
}

impl Swan {
    fn fly(&self) {
        println!("look the swan is flying ...");
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}

fn create_bird(tag: u8) -> Box<dyn Bird> {
    match tag {
        1 => Box::new(Duck),
        2 => Box::new(Swan),
        _ => panic!()
    }
}

pub fn traits_example_6() {
    let brid1 = create_bird(1);
    let brid2 = create_bird(2);
    let duck1 = Duck;

    // brid1.swim() 
    /*  this would panic even if it's underlying type is "Duck". this is cuz the trait object that was 
        specified as the return type of the function is "bird" and does not have the 'swim' method 
        included in the vtable
    */
    duck1.swim(); // this however is comletely legal and would compile
    println!("{}", brid1.quack());
    println!("{}", brid2.quack());
}

pub fn traits_example_7() {
    let duck = Duck;
    let swan = Swan;
    let birds: [&dyn Bird; 2] = [&Duck, &duck]; // can also use dynamic dispatch for arrays

    for bird in birds {
        println!("{}", bird.quack());
    }
}

trait Draw {
    fn draw(&self) -> String;
}

// you can implement user defined traits for library types
impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", self) // you don't need to derefference self with "*self" anymore
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", self) 
    }
}

fn draw_with_box(x: Box<dyn Draw>) {
    println!("{}", x.draw());
}

fn draw_with_ref(x: &dyn Draw) {
    println!("{}", x.draw());
}

pub fn traits_example_8() {
    let first = 1.6f64;
    let second = 12u8;
    draw_with_box(Box::new(first));
    draw_with_ref(&second);
}

/* Object Safe Traits
 So basically only object safe traits can be used to create trait objects.
 a trait is said to be object safe if all the methods in the trait have the following properties:
 1. the return type isn't "Self"
 2. there are no generic type parameters

*/


