use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to guess the number game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // println!("Generated Number: {secret_number}");
    
    loop{

        println!("Please Enter the Number:");
        
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the Number");
        
        let guess:u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue,
        };
        
        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("7 cr");
                break;
            },
        }
    }
}
    