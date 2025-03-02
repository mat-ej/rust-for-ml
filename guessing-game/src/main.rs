use std::io;
use std::cmp::Ordering;
use rand::Rng;

// By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude

fn main() {
    println!("Guess a number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess");

        // mutable var guess, bound to empty string for now
        let mut guess = String::new();
        // & indicates this is a reference, 
        // same as variables, references are immutable by default, hence need to use &mut
        io::stdin()
        // read_line fills guess but also returns variant Result: [Err, Ok]
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // not mutable
        let apples = 5;
        let mut bananas = 5;

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Achab"),
            Ordering::Greater => println!("Ty bzdocha vyjebana"),
            Ordering::Equal => {

                println!("Rozjebem okna aj vsetko");
                break;
            },
        }
    }
}
