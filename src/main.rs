use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guest The Number");
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess: String = String::new();

        println!("Please input your guess.");
        io::stdin().read_line(&mut guess).expect("failed read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("you guessed: {guess}");
    }
}
