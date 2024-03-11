use std::collections::HashMap;

pub fn hashmaps_example_1() {
    let mut scores: HashMap<&str, i32> = HashMap::new(); // creates an empty hashmap
    scores.insert("stanley", 78); // inserts a key-value pair into the hashmap
    scores.insert("kevin", 81); // keys and values must all be of the same type through out
    scores.insert("michelle", 79);
    scores.insert("riley", 92);
    scores.insert("suzan", 83);

    // there are 2 methods of getting values from a hashmap, using the "get" method and bracket notation

    //the get method
    let score = scores.get("kevin");
    assert_eq!(score, Some(81).as_ref());

    //the bracket notation
    if scores.contains_key("riley") {
        let score = scores["riley"];
        assert_eq!(score, 92);
        scores.remove("riley");
    }

    assert_eq!(scores.len(), 4);

    for (student, score) in scores {
        println!("the score of {} is {}", student, score);
    }
}

pub fn hashmaps_example_2() {
    let teams = [
        ("eyimba fc", 33),
        ("kwara utd", 31),
        ("mfm fc", 29),
    ];

    let mut teams1 = HashMap::new();
    for team in teams {
        teams1.insert(team.0, team.1);
    }

    let teams2: HashMap<&str, i32> = HashMap::from(teams);
    let teams3: HashMap<&str, i32> = teams.into_iter().collect();
    assert_eq!(teams1, teams2);
    assert_eq!(teams2, teams3);
    println!("hashmaps_example_2 success ...");
}

pub fn hashmaps_example_3() {
    let mut player_stats = HashMap::new();

    // the following line inserts a key if it doesn't already exist
    player_stats.entry("Health").or_insert(100);
    assert_eq!(player_stats["Health"], 100); //hashmap keys are case sensitive. "Health" diff from "health"

    // d diff btwn d above and this is that u pass in a function tha returns a value instead of a value
    player_stats.entry("Health").or_insert_with(ret_somn);
    assert_eq!(player_stats["Health"], 100);

    let mut health = player_stats.entry("Health").or_insert(52);
    assert_eq!(health, &100);
    *health -= 50;
    assert_eq!(*health, 50);

    println!("hashmaps_example_3 success ...");

    fn ret_somn() -> u8 {
        42
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Viking {
    name: String,
    country: String
}

impl Viking {
    fn new(name: &str, country: &str) -> Self {
        Self {
            name: name.to_string(),
            country: country.to_string()
        }
    }
}

pub fn hashmaps_example_4() {
    let vikings = HashMap::from([
        (Viking::new("Olaf", "Denmark"), 25),
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Harald", "Iceland"), 25),
    ]);

    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}

/*  The capacity of a hashmap works just like that of a vector, but the major difference being that 
    hashmaps can also shrink their capacity to a given size using the "shrink()" method, or can 
    shrink to fit the elements inside it with the method "shrink_to_fit()"
*/


