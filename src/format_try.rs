use ansi_term::Colour::{Green, Yellow, Red};
use std::io;
use std::io::Write;

pub fn format_tried_word(trial: &str, answer: &str) { 

    let answer_vec: Vec<char> = answer.chars().collect();

    for (i, j) in trial.chars().enumerate() {
        if answer_vec[i] == j {
            print!("{}\t", Green.paint(j.to_string()));
        } else if answer_vec.contains(&j) {
            print!("{}\t", Yellow.paint(j.to_string()));
        } else {
            print!("{}\t", Red.paint(j.to_string()));
        }
        io::stdout().flush().unwrap();
    }

    println!("");
}