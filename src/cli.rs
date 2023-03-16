use clap::Parser;

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
pub fn parse_letters() -> (char, [char; 6]) {
    todo!()
}
