struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// struct struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        // instanciation of User struct it can be mutable
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // struct update syntax, create a new struct instance from other instances
    // use ..other syntax and initate necessary fields
    let user2 = User {
        email: String::from("someone@example.com"),
        ..user1
    };
    // instanciate struct tuples
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
