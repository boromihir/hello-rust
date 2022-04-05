use ferris_says::say;

use rand::Rng;
use std::cmp::Ordering;
use std::io::{stdin, stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("secret number is: {}", secret_number);

    loop {
        println!("input your guess: ");

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Winner winner!");
                break;
            }
        }

        println!("\n")
    }
}
