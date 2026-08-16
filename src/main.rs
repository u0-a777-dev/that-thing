use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!("\n🎲 Welcome to the Mystery Game! 🎲");
    println!("You have 7 attempts to guess the secret number.");
    println!("The number is between 1 and 100.\n");

    let mut rng = rand::thread_rng();
    let secret = rng.gen_range(1..=100);
    let mut attempts = 0;
    let max_attempts = 7;

    loop {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Please enter a valid number!");
                continue;
            }
        };

        attempts += 1;

        if guess == secret {
            println!("\n🎉 You got it! The number was {}!", secret);
            println!("You solved it in {} attempt{}!\n", attempts, if attempts == 1 { "" } else { "s" });
            break;
        } else if guess < secret {
            println!("📈 Too low! Try higher.");
        } else {
            println!("📉 Too high! Try lower.");
        }

        let remaining = max_attempts - attempts;
        println!("Attempts remaining: {}\n", remaining);

        if attempts >= max_attempts {
            println!("\n💀 Game Over! The secret number was {}.", secret);
            println!("Better luck next time!\n");
            break;
        }
    }
}
