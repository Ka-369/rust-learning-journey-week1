/// Demonstrates how ownership works in Rust with Strings (heap data)
/// and integers (stack data). Run this to understand the move vs copy behavior.

fn main() {
    // Strings are stored on the heap.
    // When passed to a function, ownership is moved unless a reference is used.
    let s = String::from("Welcome, World");

    // Ownership of `s` is transferred to the function.
    // After this, `s` is invalid — uncommenting the next line will cause an error.
    // println!("{}", s);

    let t = change_owner(s);
    println!("Back in main with t: {}", t);

    // Integers implement the Copy trait.
    // Passing them to functions copies the value — ownership stays in main.
    let x: i32 = 54;
    let y = change_owner_num(x);
    println!("x: {}, y: {}", x, y);
}

fn change_owner(u: String) -> String {
    println!("Inside change_owner(): {}", u);
    u // Return ownership
}

fn change_owner_num(z: i32) -> i32 {
    println!("Inside change_owner_num(): {}", z);
    z
}
