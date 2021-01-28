struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email:String, username:String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}


fn main() {
    let user1 = User {
        username: String::from("someone"),
        email: String::from("s@s.com"),
        active: true,
        sign_in_count: 1,
    };
    let u = build_user(String::from("hoge"), String::from("huga"));

    let user2 = User {
        username: String::from("a"),
        email: String::from("b"),
        ..user1
    };
}
