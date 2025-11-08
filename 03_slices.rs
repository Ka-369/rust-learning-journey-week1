fn main() {
    // Example 1: String slice – get first 3 characters from "Welcome"
    let r = String::from("Welcome");
    let s = &r[0..3]; // Slices the string from index 0 to 2, returns "Wel"
    println!("{}", s);

    // Example 2: Array slice – iterate over all elements in an array
    let x = [100, 300, 500];
    let y = &x[..]; // Slices the entire array, type: &[i32]
    for i in y {
        println!("{}", i); // Prints each element in the slice
    }

    // Example 3: Mutable slice – update elements in a mutable slice
    let mut a = [1, 2, 3];
    let b = &mut a[..]; // Get a mutable slice of the array
    b[0] = 3;           // Change first element to 3
    b[2] = 1;           // Change last element to 1
    println!("{:?}", a); // Print the entire updated array

    // Example 4: Slicing and splitting – create smaller slices from an array
    let k = [1, 3, 5, 7, 9];
    let l = &k[0..4]; // Slice contains first 4 elements: [1, 3, 5, 7]
    let m = &l[0..2]; // Slice first 2 elements from above: [1, 3]
    println!("{:?}", k); // Print original array
    println!("{:?}", l); // Print first slice
    println!("{:?}", m); // Print slice of a slice

    // Example 5: Slice manipulation – using methods on slices
    let h = [1, 2, 3, 4, 5, 6, 7];
    let i = &h[..]; // Slice whole array
    println!("Length : {}", i.len()); // Show slice length
    println!("Contains 9? {}", i.contains(&9)); // Check for value, will be false
    println!("First: {:?}", i.first()); // Print first element (Option<&i32>)
    println!("Last: {:?}", i.last()); // Print last element (Option<&i32>)
    println!("Empty? {}", i.is_empty()); // Check if slice is empty
}
