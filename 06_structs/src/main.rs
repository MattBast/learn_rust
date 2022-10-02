// standard structs are declared at the start of the program
// and contain definitions of the stricts fields
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i32
}

// tuples can also be defined as structs. They behave the 
// same except that they don't have fields.
struct Color(i32, i32, i32);

fn main() {
    // a struct can then be used to create a variable
    // adding the mut flag makes the fields editable
    // otherwise the variable would be immutable.
    let mut user1 = User {
        email: String::from("matt@domain.com"),
        username: String::from("Matt"),
        active: true,
        sign_in_count: 0
    };

    // dot notation can be used to access the variables fields
    // for both reading and writing
    user1.email = String::from("matthew@domain.com");

    println!(
        "User {}s status is {} and signed in {} times.", 
        user1.username, 
        user1.active,
        user1.sign_in_count
    );

    // functions can also be used to create variables
    // using a struct
    let user2 = create_user(
        "Lucy".to_string(), 
        "lucy@domain.com".to_string()
    );

    println!(
        "User {}s status is {} and signed in {} times.", 
        user2.username, 
        user2.active,
        user2.sign_in_count
    );

    // another shortcut that can be used is to copy over
    // another structs values except the specified values.
    // the .. syntax achieves this.
    let user3 = User {
        email: String::from("sam@domain.com"),
        username: String::from("Sam"),
        ..user2
    };

    println!(
        "User {}s status is {} and signed in {} times.", 
        user3.username, 
        user3.active,
        user3.sign_in_count
    );

    // define a tuple struct like this
    let black = Color(0, 0, 0);
    println!(
        "The colour black is defined as: ({},{},{})", 
        black.0, 
        black.1, 
        black.2
    );
}

fn create_user(username: String, email: String) -> User {
    return User {
        email, // 'email: email' can also be used set the variable
        username, // but since the variable and field names are the same we can use this shortcut
        active: false,
        sign_in_count: 1
    };
}