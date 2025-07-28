fn main() {
    // Using mutability
    let mut count = 1;
    count = 2; // Mutating the variable
    println!("Count using mutability: {}", count); // Outputs: 2

    // Using shadowing
    let count = "one"; // Shadowing with a different type
    let count = count.len(); // Shadowing again with a different type
    println!("Count using shadowing: {}", count); // Outputs: 3
}