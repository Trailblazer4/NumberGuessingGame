use std::io;
use rand::Rng;
use colored::*;
use std::cmp::Ordering;

fn user_guess(guess: &mut String) -> i32 {
    loop {
        println!("{}", "Guess a number: ".bright_cyan());

        io::stdin()
        .read_line(guess)
        .expect("Failed to read line");

        match guess.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("{} {}{}", "please enter a".red(), "number".purple().italic(), "!".red());
                guess.clear();
                continue;
            },
        };
    }
}

fn main() {
    // let randy: i32 = 1 + rand::random::<i32>().abs() % 10;
    let randy: i32 = rand::thread_rng().gen_range(1, 100);

    let mut guess: String = String::new();
    
    for _ in 0..7 {
        let guess_i: i32 = user_guess(&mut guess);

        println!("Your guess was {}.", &guess_i);

        match guess_i.cmp(&randy) {
            Ordering::Less => println!("{}\n{}", "D'oh! Try again!".red(), "Hint: go right".yellow()),
            Ordering::Greater => println!("{}\n{}", "D'oh! Try again!".red(), "Hint: go left".yellow()),
            Ordering::Equal => {
                println!("{}", "Wow, you're really good at this!".green());
                break;
            },
        }

        guess.clear();
    }
    println!("The answer was {}.", randy);
}
