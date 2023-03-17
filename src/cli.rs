use clap::Parser;
use color_eyre::eyre::Result;
use dialoguer::Input;

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
    let middle = args.middle.map(Ok).unwrap_or_else(prompt_for_middle)?;

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

    Ok(chosen)
}

/// Prompts for the other letters if they have not been entered. Makes sure they are
/// not the middle letter.
fn prompt_for_others(middle: char) -> Result<[char; 6]> {
    loop {
        let chosen: String = Input::new()
            .with_prompt("Enter the letters on the outer ring of the hexagon, separated by a space")
            .interact()?;

        let chosen = chosen.split_whitespace().collect::<Vec<_>>();

        // Rewrite the above code to use a match statement.
        match chosen.len() {
            6 => (),

            // Greater than 6
            _ if chosen.len() > 6 => {
                println!("Too many letters, please try again.");
                continue;
            }

            // Less than 6
            _ => {
                println!("Not enough letters, please try again.");
                continue;
            }
        }

        let mut others = vec![];

        for letter in chosen {
            if letter.len() > 1 {
                println!("'{}' is not a letter, please try again.", letter);
                continue;
            }

            // TODO: Make letter sure it is not the middle letter.
            if letter.starts_with(middle) {
                println!("'{}' is the middle letter, please try again.", letter);
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
