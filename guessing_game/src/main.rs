use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    // print welcome msg
    println!("Welcome to the guessing game!");

    // generate secret number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // run loop
    loop {
        
        // make and input guess
        let mut guess = String::new();
        println!("Please enter a guess!");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert guess to int
        let guess: u8 = guess.trim().parse().expect("Pls enter a number");

        // print guess
        println!("You guessed: {}", guess);

        // compare guess to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),

            // end loop if guess is correct
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
