fn main() {
    let mut count = 0;

    loop {
        count += 1;
        println!("Count: {}", count);

        if count == 20 {
            break;
        }
    }

    println!("Final count is {}", count);
}