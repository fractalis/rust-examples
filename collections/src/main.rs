use std::collections::HashMap;

fn main() {
    let _v: Vec<i32> = Vec::new();

    let mut v2 = vec![1,2,3,4,5];

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    };

    for i in &v2 {
        println!("{}", i);
    }

    for i in &mut v2 {
        *i += 50
    }

    for i in &v2 {
        println!("{}", i);
    }

    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    let mut s2 = String::from("foo");
    s2.push_str("bar");

    let s3 = String::from("Hello, ");
    let s4 = String::from("World!");
    let s5 = s3 + &s4; // Note s3 has been moved here and can no longer be used

    let s6 = String::from("tic");
    let s7 = String::from("tac");
    let s8 = String::from("toe");
    let s9 = format!("{}-{}-{}",s6,s7,s8);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    scores.entry(String::from("Blue")).or_insert(50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
