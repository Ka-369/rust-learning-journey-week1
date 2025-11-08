/// Demonstrates mutable and immutable references in Rust.
///
/// - `update_str` shows how to mutate a String using a mutable reference.
/// - `immut_update_str` shows how to read data using an immutable reference.

fn main() {
    let mut a = String::from("Welcome");
    println!("Before changing: {}", a);
    update_str(&mut a);
    println!("After changing: {}", a);

    let c = String::from("Immutable Str");
    immut_update_str(&c);
}

fn update_str(b: &mut String) {
    b.push_str(" world");
}

fn immut_update_str(d: &String) {
    println!("{}", d);
}
