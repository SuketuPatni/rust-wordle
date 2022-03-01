use rand::prelude::*;
use std::io;

mod format_try;

const TRIES:&str = include_str!("../possible_tries.txt");
const ANSWERS: &str = include_str!("../possible_answers.txt");

fn input_word() ->  String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("ERROR: Failed to read line");

    input.trim().to_ascii_lowercase()
}

fn main() {
    let mut tries = [""; 5757];
    let mut answers = [""; 2314];

    for (i, j) in TRIES.lines().enumerate() {
        tries[i] = j;
    }

    for (i, j) in ANSWERS.lines().enumerate() {
        answers[i] = j;
    }

    // let chosen_index = thread_rng().gen_range(0..2313);
    // let chosen_word = answers[chosen_index];
    let chosen_word = "gross";

    println!("Welcome to Wordle!");

    let mut trial = input_word();

    let mut num_tries = 0;

    loop {

        if tries.contains(&(trial.as_str())) == false {
            println!("ERROR: Not a valid try!");
        } else {
            format_try::format_tried_word(&trial.as_str(), chosen_word);
            
            if num_tries == 6 {
                println!("Your 6 tries are over!");
                // The correct word was {}", chosen_word
                if trial == chosen_word {
                    println!("Congratulations! You won in {} tries", num_tries + 1);
                    break;
                } else {
                    println!("You lost. Try again!");
                }
                break;
            } 

            num_tries += 1;
        }

        trial = input_word();
    }

    
}

