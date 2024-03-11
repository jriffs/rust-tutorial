#[derive(Debug)] // this is used so that the struct can be printed to the output/terminal
struct User {
    active: bool,
    name: String,
    email: String,
    uid: String,
    sign_in_count: u32
}

pub fn struct_example_1() {
    let mut user1 = User{
        active: false,
        name: "Jeremiah".to_string(),
        email: String::from("myemail123@gmail.com"),
        uid: "f7gfa8as8f7asdfas9d8fsad9f7sd".to_string(),
        sign_in_count: 1
    };
    user1.active = true; // this is only possible cuz user1 is declared as mutable
    if user1.active == true {
        println!("it's all true");
    } else {
        println!("it's all a lie");
    }
}

pub fn struct_example_2() {
    let user1 = User{
        active: false,
        name: "Jeremiah".to_string(),
        email: String::from("myemail123@gmail.com"),
        uid: "f7gfa8as8f7asdfas9d8fsad9f7sd".to_string(),
        sign_in_count: 1
    };

    let user2 = User {
        name: String::from("Nicky"),
        email: String::from("something@gmail.com"),
        uid: "df76fd98g76dsf87g69fdsz87g".to_string(),
        ..user1 // this line just simply means take everything else from user1
    };

    assert_eq!(user1.active, user2.active);
    println!("success");
    println!("{:?}", user2);
}

pub fn struct_example_3() {
    // tuple struct
    /* it's basically a named tuple */
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}




