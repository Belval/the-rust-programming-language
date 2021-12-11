struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // User 1 is immutable
    let user1 = User {
        active: true,
        username: String::from("blah"),
        email: String::from("bleh"),
        sign_in_count: 1,
    };

    // User 2 is mutable
    let mut user2 = User {
        active: true,
        username: String::from("bloh"),
        email: String::from("blih"),
        sign_in_count: 1,
    };

    user2.sign_in_count = 24;

    println!("User 2 signin count is: {}", user2.sign_in_count);

    let user3 = build_user("Test".to_string(), "test@test.com".to_string());
}
