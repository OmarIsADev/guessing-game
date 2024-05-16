// use std::io;
use std::io::stdin;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a the Secret number between 1 and 10");
    
    loop {
        println!("Guess a number!");
        println!("Type `quit` to exit the game");
        
        let mut guess: String = String::new(); // mut means mutable or changeable
        let secret_number: u32 = rand::thread_rng().gen_range(1..=10);
    
        stdin() // or we can use io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // guess == "quit" breaks the loop
        if guess.trim() == "quit" {
            println!("Goodbye!");
            break;
        }
        
        println!("You guessed: {guess}");

        // trim() removes white spaces and parse() converts string to number
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                println!("__________________________");
                continue;
            }
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
                }
        }

        println!("__________________________");
        println!();
    }
    
}
