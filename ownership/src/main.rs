fn main() {

    {
                           // s is not valid here, not declared
        let _s = "hello";   // s is valid from this point forward
        // do stuff with s
    }   // the scope is now over, and s is no longer valid

    let s1 = String::from("Hello");
    let mut s2 = s1.clone();

    s2.push_str(", World!");

    println!("s1 = {}, s2 = {}", s1, s2);

    let s3 = gives_ownership();

    let mut s4 = takes_and_gives_back(s3);

    println!("S4: {}", s4);

    let str_len: usize = calculate_length(&s4);
    println!("S4 Length: {}", str_len);

    change(&mut s4);
    println!("S4: {}", s4);
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope
    a_string // a_string is returned and moves out to the calling function
}
