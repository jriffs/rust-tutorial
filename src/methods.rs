#[derive(Debug)]
/* methods and associated function are both created using the impl keyword, but they differ in 2 ways:
    1. methods are called with a self parameter as the first parameter while AF's are not
    2. methods can be caled on the instances of the struct/enum while AF's cannot, they're called on the
    struct/enum identifier themselves
*/

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

struct TrafficLight {
    color: String
}

impl TrafficLight {
    // here "Self" is the struct itself, so "&Self" (which is the same as &self) is a borrowed 
    // instance of the struct
    fn new() -> Self {
        Self {
            color: String::from("red")
        }
    }
    pub fn show_state(self: &Self) -> () {
        println!("the state of the traffic light is: {}", self.color);
    }
}
// You can have more than 1 impl block for a struct/enum
impl TrafficLight {
    // here a mutable refference to self is used cuz we will be mutating a field
    // because these are functions, we use a refference to "self" so that the functions don't take 
    // ownership of the instance and we can use them later.
    pub fn change_color(&mut self) {
        self.color = "green".to_string();
    }
}

enum TrafficLightColor {
    Red,
    Yellow,
    Green
}

impl TrafficLightColor {
    fn color(&self) -> &str {
        match self {
            Self::Green => "green",
            Self::Red => "red",
            Self::Yellow => "yellow",
        }
    }
}

pub fn methods_example_1() {
    // now you can create an instance of Rectangle in 2 ways
    let rect1 = Rectangle {
        width: 20,
        height: 10
    };
    let rect2 = Rectangle::create(20, 10);
    assert_eq!(rect1.width, rect2.width);
    println!("rect1.width: {}, is equal to rect2.width: {}", rect1.width, rect2.width);
}

pub fn methods_example_2() {
    let tl1 = TrafficLight {
        color: String::from("red")
    };
    tl1.show_state();
    // we also have to declare the variable as mutable to use the "change_color"
    let mut tl2 = TrafficLight::new();
    // now you can call tl2.change_color()
    tl2.change_color();
}

pub fn methods_example_3() {
    let c = TrafficLightColor::Yellow;
    assert!(c.color() == "yellow");
    println!("it is yellow");
}