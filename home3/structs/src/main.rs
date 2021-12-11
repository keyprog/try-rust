
fn main() {
    println!("Hello, world!");
    let user1 = User {
        username: String::from("aledx"),
        email: String::from("test@test.com"),
        sign_in_count: 0,
        active: true
    };

    let user2 = build_user(String::from("test"), String::from("2"));

    let user3 = User {
        ..user2
    };
    println!("User: {:?}", user1);
    println!("Test: {}", user1.test(false));

    struct Color(i32, i32, i32);
    let color = Color(1, 2, 3);

    println!("{}", User::test2(true));


}
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

impl User {
    pub fn test(&self, some: bool) -> bool {
        return self.active && some;
    }
}

impl User {
    pub fn test2(flag: bool) -> bool {
        flag
    }
}


fn build_user(email: String, username: String) -> User{
    User{
        username,
        email,
        active: true,
        sign_in_count: 1
    }
}
