fn main() {
    test_struct();
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn test_struct() {
    let mut u1 = User {
        email: String::from("123@example.com"),
        username: String::from("mn"),
        active: true,
        sign_in_count: 0,
    };
    u1.sign_in_count += 1;

    let mut u2 = build_user(String::from("1@1.com"), String::from("mn"));
    u2.sign_in_count += 1;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
