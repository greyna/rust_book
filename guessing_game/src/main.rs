use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    song();
    //fibonacci(10);
    //degrees();
    //guess_the_number();
}

pub fn song() {
    let day_ordinals = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "10th", "11th", "12th",
    ];
    let day_quote = ["On the ", " day of Christmas my true love gave to me"];
    let items = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    let format_item = |item_index, item, day_index| {
        let divider = match item_index {
            0 => ".\n",
            1 if day_index > 1 => ", and ",
            _ if item_index % 2 == 0 => ",\n",
            _ => ", ",
        };
        format!("{}{}", item, divider)
    };

    let format_paragraph = |(day_index, day_ordinal)| {
        let items_string = items
            .iter()
            .enumerate()
            .take(day_index + 1)
            .map(|(index, &item)| format_item(index, item, day_index))
            .rev()
            .fold(String::new(), |acc, item| format!("{}{}", acc, item));

        format!(
            "{}{}{}\n{}",
            day_quote[0],
            day_ordinal,
            day_quote[1],
            uppercase_first_letter(items_string)
        )
    };

    let text: String = day_ordinals
        .iter()
        .enumerate()
        .map(format_paragraph)
        .fold(String::new(), |text, paragraph| {
            format!("{}\n{}", text, paragraph)
        });

    println!("\n\t--- Twelve Days of Christmas ---\n{}", text);
}

fn uppercase_first_letter(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn fibonacci(n: u32) {
    println!("\n\t--- FIBONACCI ---");
    println!("Fibonacci({}) = {}", n, fibo_iter(n));
}

fn _fibo_rec(n: u32) -> u128 {
    // much less optimized, overflows the stack easily
    match n {
        0 => 0,
        1 => 1,
        _ => _fibo_rec(n - 1) + _fibo_rec(n - 2),
    }
}

fn fibo_iter(n: u32) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut fibo_min_2 = 0; // fibo(0)
            let mut fibo_min_1 = 1; // fibo(1)
            let mut result: u128 = 1; // fibo(1)
            for _ in 2..n {
                result = fibo_min_1 + fibo_min_2; // fibo(i)
                fibo_min_2 = fibo_min_1;
                fibo_min_1 = result;
            }
            result
        }
    }
}

pub fn degrees() {
    println!("\n\t--- DEGREES ---");

    let temp_c = DegreesC(20f32);
    println!(
        "It's {} degrees C, so {} degrees F.",
        temp_c.0,
        convert_c_to_f(temp_c).0
    );

    let temp_f = DegreesF(77f32);
    println!(
        "It's {} degrees F, so {} degrees C.",
        temp_f.0,
        convert_f_to_c(temp_f).0
    );
}

#[derive(Debug, Clone, Copy)]
struct DegreesF(f32);
#[derive(Debug, Clone, Copy)]
struct DegreesC(f32);

fn convert_c_to_f(deg: DegreesC) -> DegreesF {
    DegreesF(deg.0 * 9f32 / 5f32 + 32f32)
}

fn convert_f_to_c(deg: DegreesF) -> DegreesC {
    DegreesC((deg.0 - 32f32) * 5f32 / 9f32)
}

pub fn guess_the_number() {
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
