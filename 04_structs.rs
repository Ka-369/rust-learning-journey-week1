// This is a simple struct to hold info about a user.
// We're keeping track of their username, email, whether their account is active, and how many times they've signed in.
struct User {
    username: String,     // The user's name they'll use to log in.
    email: String,        // Where we can contact the user, or send notifications.
    active: bool,         // Is the account currently being used?
    sign_in_count: u64,   // How many times this user has logged in.
}

fn main() {
    // Let's create a sample user named "luffy@123".
    // We're giving them an email and setting their account as active.
    // We'll also say they've signed in 3 times so far.
    let user1 = User {
        username: String::from("luffy@123"),
        email: String::from("luffy@gmail.com"),
        active: true,
        sign_in_count: 3,
    };

    // Now, let's print out their username to check that everything works!
    println!("user1 username: {}", user1.username);
}
