mod cli;
mod solver;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    // Initialize color-eyre
    color_eyre::install()?;

    // Get the letters required.
    let (middle, others) = cli::parse_letters()?;

    // Get the words
    let words = solver::get_possible_words(middle, others);

    // If there are no words, print an error message.
    if words.is_empty() {
        eprintln!("No possible words! Try checking your input.")
    }
    // Otherwise, print them out!
    else {
        println!("Possible words: ");

        for word in words {
            println!("{}", word);
        }
    }

    Ok(())
}
