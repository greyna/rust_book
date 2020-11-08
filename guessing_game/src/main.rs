use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\n\t--- GUESS THE NUMBER ---");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Please input your guess.");

    loop {
        let mut guess_str = String::new();

        if io::stdin().read_line(&mut guess_str).is_err() {
            println!("io error while reading stdin\n");
            continue;
        }

        let guess_nb: i32 = match guess_str.trim().parse() {
            Ok(nb) => nb,
            Err(_) => {
                println!("Please type a number!\n");
                continue;
            }
        };

        match guess_nb.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
