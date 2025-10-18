use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
println!("Guess a number!");

let secret_number:u32 = rand::thread_rng().gen_range(1..=200);
println!("The secret number is {}", secret_number);

loop {
    println!("Please input your guess.");
    let mut put_in_your_guess = String::new();
    io::stdin()
        .read_line(&mut put_in_your_guess)
        .expect("failed to read line");
    let put_in_your_guess: u32 = match put_in_your_guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("Your guess was    {}", put_in_your_guess);

    match put_in_your_guess.cmp(&secret_number) {
        Ordering::Less => println!("{}","Your input is too small".red()),
        Ordering::Greater => println!("{}","Your input is too big".yellow()),
        Ordering::Equal => {
            println!("{}","You're a genius!".green());
            break;
        }
    }
}
}
