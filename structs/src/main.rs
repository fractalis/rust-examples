struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height
    }
}

fn main() {

    let user1 = User {
        email: String::from("someuser@example.com"),
        username: String::from("someusername123"),
        sign_in_count: 1,
        active: true
    };

    println!("User (email: {}, username: {} sign_in_count: {} active: {})", user1.email, user1.username,
                                                                            user1.sign_in_count, user1.active);

    let user2 = build_user(String::from("someotheruser"), String::from("someotheruser@example.com"));
    println!("User (email: {}, username: {} sign_in_count: {} active: {})", user2.email, user2.username,
                                                                            user2.sign_in_count, user2.active);

    let user3 = User {
        email: String::from("yetanotheruser@example.com"),
        username: String::from("yetanotheruser"),
        ..user2
    };
    println!("User (email: {}, username: {} sign_in_count: {} active: {})", user3.email, user3.username,
                                                                            user3.sign_in_count, user3.active);

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3: {}", rect1.can_hold(&rect3));

    println!("rect1 is {:#?}", rect1);

}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true
    }
}
