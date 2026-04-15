use rand;
use std::io;

fn main() {
    let random_value = rand::random_range(0..6);
    const CHANCES: usize = 3;

    for _ in 0..CHANCES {
        println!("Enter your prediction: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read");

        let prediction: u32 = input.trim().parse().expect("not a number");

        println!("your input: {}", prediction);

        if random_value == prediction {
            println!("You won!");
            std::process::abort()
        }
    }

    println!("You lost!, magic number: {}", random_value);
}
