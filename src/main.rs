use std::cmp::Ordering;
use std::io;
use rand::Rng;


fn main() {
    println!("Guess the Number!");

    let secret_guess = rand::thread_rng().gen_range(10..=100);

    println!("Please input your guess");

    loop {
        
        let mut user_guess = String::new();

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to Read Line");
        
        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is {user_guess}");
        
        match user_guess.cmp(&secret_guess) {
            Ordering::Less => println!("Too Small !"),
            Ordering::Greater => println!("Too Big !"),
            Ordering::Equal => {
                println!("You Found the secret number !");
                break;
            }
       }
    }
}


