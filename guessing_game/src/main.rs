use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    //let data = [3, 1, 1, 2, 1];
    //dbg!(stats(&data));
    song();
    //fibonacci(10);
    //degrees();
    //guess_the_number();
}

pub fn stats(data: &[i32]) -> (f32, f32, i32) {
    let mean = data.iter().sum::<i32>() as f32 / data.iter().count() as f32;

    let mut occurences = HashMap::new();
    for &nb in data {
        let entry = occurences.entry(nb).or_insert(0);
        *entry += 1;
    }
    let (mode, _) = occurences
        .into_iter()
        .max_by_key(|(_, occurences)| *occurences)
        .unwrap();

    let mut data = data.to_owned();
    data.sort_unstable();
    let count = data.iter().count();
    let median = match count {
        _ if count % 2 == 0 => {
            data.into_iter()
                .take((count / 2) + 1)
                .rev()
                .take(2)
                .sum::<i32>() as f32
                / 2f32
        }
        _ => data.into_iter().take((count / 2) + 1).last().unwrap() as f32,
    };

    (mean, median, mode)
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

    let get_divider = |item_index, day_index| match item_index {
        0 => ".\n",
        1 if day_index > 1 => ", and ",
        _ if item_index % 2 == 0 => ",\n",
        _ => ", ",
    };

    let get_items_sentence = |day_index| {
        let items_sentence = items
            .iter()
            .enumerate()
            .take(day_index + 1)
            .map(|(index, &item)| (item, get_divider(index, day_index)))
            .rev();

        let mut sentence = String::new();
        for (item, divider) in items_sentence {
            write!(&mut sentence, "{}{}", item, divider).unwrap();
        }
        
        uppercase_first_letter(sentence)
    };

    let mut text = String::from("\n\t--- Twelve Days of Christmas ---\n");
    for (day_index, &day_ordinal) in day_ordinals.iter().enumerate() {
        write!(
            &mut text,
            "\n{}{}{}\n{}",
            day_quote[0],
            day_ordinal,
            day_quote[1],
            get_items_sentence(day_index)
        )
        .unwrap();
    }

    println!("{}", text);
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
