use crossterm::{
    execute,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
};
use std::io::{stdout, Error};

pub fn format_tried_word(trial: &str, answer: &str) -> Result<(), Error> {
    let answer_vec: Vec<char> = answer.chars().collect();

    for (i, j) in trial.chars().enumerate() {
        let formatted_char = format!(" {} \t", j);
        if answer_vec[i] == j {
            execute!(
                stdout(),
                SetBackgroundColor(Color::DarkGreen),
                SetForegroundColor(Color::White),
                SetAttribute(Attribute::Bold),
                Print(formatted_char),
                ResetColor
            )?;
        } else if answer_vec.contains(&j) {
            execute!(
                stdout(),
                SetBackgroundColor(Color::DarkYellow),
                SetForegroundColor(Color::White),
                SetAttribute(Attribute::Bold),
                Print(formatted_char),
                ResetColor
            )?;
        } else {
            execute!(
                stdout(),
                SetBackgroundColor(Color::DarkRed),
                SetForegroundColor(Color::White),
                SetAttribute(Attribute::Bold),
                Print(formatted_char),
                ResetColor
            )?;
        }
    }

    println!();

    Ok(())
}
