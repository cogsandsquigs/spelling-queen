use clap::Parser;
use color_eyre::eyre::Result;
use dialoguer::{Input, MultiSelect};

/// An automatic solver for the New York Times' Spelling Bee game, built in Rust.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The middle letter for the puzzle.
    #[arg(short, long)]
    middle: Option<char>,

    /// The other extraneous letters. There should only be 6 (no more or less), and
    /// they should be separated by commas.
    #[arg(short, long, value_parser, num_args = 6)]
    others: Option<Vec<char>>,
}

/// Parse the arguments from the user and get the middle and ext. letters.
pub fn parse_letters() -> Result<(char, [char; 6])> {
    // Parse CLI arguments.
    let args = Cli::parse();

    // Get middle character.
    let middle = args
        .middle
        .map(|middle| Ok(middle))
        .unwrap_or_else(prompt_for_middle)?;

    // Get other characters
    let others = args
        .others
        .map(|others| {
            Ok(others
                .try_into()
                .expect("Clap should only parse 6 letters, no more no less!"))
        })
        .unwrap_or_else(|| prompt_for_others(middle))?;

    Ok((middle, others))
}

/// Prompts for the middle letter if it has not been entered.
fn prompt_for_middle() -> Result<char> {
    let chosen: char = Input::new()
        .with_prompt("Enter the letter in the middle of the hexagon")
        .interact()?;

    return Ok(chosen);
}

/// Prompts for the other letters if they have not been entered. Makes sure they are
/// not the middle letter.
fn prompt_for_others(middle: char) -> Result<[char; 6]> {
    // let letters: Vec<char> = ('a'..'z').into_iter().filter(|c| *c != middle).collect();

    // let mut chosen: Vec<usize> = vec![];

    // while chosen.len() != 6 {
    //     chosen = MultiSelect::new()
    //         .items(&letters)
    //         .with_prompt("Select the letters on the outer ring of the hexagon")
    //         .interact()?;

    //     if chosen.len() < 6 {
    //         println!("Please select more letters (only 6).")
    //     } else if chosen.len() > 6 {
    //         println!("Please select less letters (only 6).")
    //     }
    // }

    // Ok(chosen
    //     .iter()
    //     .map(|idx| letters[*idx])
    //     .collect::<Vec<_>>()
    //     .try_into()
    //     .expect("There should only be six characters here!"))

    loop {
        let chosen: String = Input::new()
            .with_prompt("Enter the letters on the outer ring of the hexagon, separated by a space")
            .interact()?;

        let chosen = chosen.split_whitespace().collect::<Vec<_>>();

        // Report bad input.
        if chosen.len() > 6 {
            println!("Too many letters, please try again.");
            continue;
        } else if chosen.len() < 6 {
            println!("Not enough letters, please try again.");
            continue;
        }

        let mut others = vec![];

        for letter in chosen {
            if letter.len() > 1 {
                println!("'{}' is not a letter, please try again.", letter);
                continue;
            }

            others.push(
                letter
                    .chars()
                    .next()
                    .expect("There should be at least one character!"),
            );
        }

        return Ok(others.try_into().expect("This should convert correctly!"));
    }
}
