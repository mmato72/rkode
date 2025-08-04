fn main() {
    let number = 25;

    if number % 5 == 0 && number % 2  == 0 {
        println!("The number is divisible by both 5 and 2");
    }
    else if number % 5 == 0 {
        println!("The number is divisible by 5 but not by 2");
    }
    else {
        println!("The number is not divisible by 5");
    }
}