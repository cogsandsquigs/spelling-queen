use lazy_static::lazy_static;

lazy_static! {
    static ref WORD_LIST: Vec<&'static str> = get_words(4);
    static ref WORD_FLAGS: Vec<u32> = get_word_bitflags();
}

/// Get all of the words. Filters out any words that do not meet the minimum length. i.e.,
/// if `min_length` is 4, then words less than 4 characters are filtered out, while words
/// of length 4 or greater are kept. Note that all words are lowercase.
/// TODO: Make this compile-time
/// TODO: Make this only nytimes words
fn get_words(min_length: usize) -> Vec<&'static str> {
    include_str!("words_alpha.txt")
        .lines()
        .filter(|word| word.len() >= min_length)
        .collect()
}

/// Another list, using bitflags to show what letters they contain. Each entry is
/// one `u32`, with the LSB representing the letter A, next bit for letter B, etc.
/// TODO: Make this compile-time
fn get_word_bitflags() -> Vec<u32> {
    WORD_LIST
        .iter()
        .map(|word| {
            word.chars()
                .map(|c| 1_u32 << c as u32 - 'a' as u32)
                .fold(0, |acc, x| acc | x)
        })
        .collect()
}

fn main() {
    println!("{:?}", &WORD_LIST[0..3]);
    println!("{:?}", &WORD_FLAGS[0..3]);
    println!("Hello, world!");
}
