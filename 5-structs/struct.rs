struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let mut user: User = User {
        username: String::from("ShivanshuMishra"),
        email: String::from("sm1011200109@gmail.com"),
        sign_in_count: 1,
        active: true
    };

    let name = user.username;
    println!("username: {}", name);

    user.username = String::from("Shivanshu10");
    println!("username: {}", user.username);

    let user2 = build_user(String::from("derek23@gmail.com"), String::from("derek"));

    // set remaining fields from other struct
    let user3 = User {
        email: String::from("james123@gmail.com"),
        username: String::from("James"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    return User {
        email: email,
        username: username,
        sign_in_count: 0,
        active: false
    };
}