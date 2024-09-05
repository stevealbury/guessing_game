//import from library file
//note the Rng is not as per book
use rand::thread_rng;
use rand::Rng;
use std::cmp::Ordering;

use std::io;

//fn main needed in all projects4
fn main() {
    println!("Guess the number!");

    //generate a random number
    let secret_number = thread_rng().gen_range(1..=100);

    //print out when testing comparison
    //println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        //can use for testing
        //println!("The secret number was: {secret_number}");
    }
}
