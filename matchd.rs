fn main() {
    let pair = (2, 3);

    match pair {
        (x, y) if x == y => println!("The numbers are equal"),
        (x, y) if x + y == 0 => println!("The numbers are opposites"),
        (x, y) if x + y > 0 => println!("The sum of the numbers is positive"),
        _ => println!("No special properties"),
    }
}