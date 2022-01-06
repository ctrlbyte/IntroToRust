use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // generate random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Guess the number!");

    loop {
        println!("Please input your guess: ");

        // creates mutable variable `guess`
        let mut guess = String::new();

        // read one line and store it in `guess`
        io::stdin()
            .read_line(&mut guess)
            .expect("Faied to read line");

        // trim() -> removes \n or \r\n
        // parse() -> parses string into number (in this case -> u32)
        // expect() -> helper functions to
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        // compares guess to secret number
        // `match` expression contains `arms`, which consist of
        //      patterns and code to run if pattern matches.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
