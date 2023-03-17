use lazy_static::lazy_static;

lazy_static! {
    static ref WORD_LIST: Vec<String> = get_words(4);
    static ref WORD_FLAGS: Vec<u32> = get_word_bitflags();
}

/// Get all of the words. Filters out any words that do not meet the minimum length. i.e.,
/// if `min_length` is 4, then words less than 4 characters are filtered out, while words
/// of length 4 or greater are kept. Note that all words are lowercase.
/// TODO: Make this compile-time
/// TODO: Make this only nytimes words - currently, its scrabble words?
fn get_words(min_length: usize) -> Vec<String> {
    include_str!("collins_scrabble_words_2019.txt")
        .lines()
        .filter(|word| word.len() >= min_length)
        // Standardize word case.
        .map(|word| word.to_lowercase())
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
                .map(char_to_bitflag)
                // OR them all together to create one big bitflag.
                .fold(0, |acc, x| acc | x)
        })
        .collect()
}

/// Converts an ASCII character into a bitflag.
fn char_to_bitflag(c: char) -> u32 {
    1_u32 << (c as u32 - 'a' as u32)
}

/// Gets all the possible words that could be made with both the middle letter and the other
/// required 6 letters. All characters MUST be lowercase.
pub fn get_possible_words(middle: char, others: [char; 6]) -> Vec<&'static str> {
    // Precomputatiton
    let middle_flag = char_to_bitflag(middle);
    let others_flags = others
        .into_iter()
        .map(char_to_bitflag)
        .fold(0, |acc, x| acc | x);

    WORD_FLAGS
        .iter()
        .enumerate()
        // Remove all words without the middle character.
        .filter(|(_, flags)| *flags & middle_flag != 0)
        // Remove all words without any of the other characters.
        .filter(|(_, flags)| *flags & !(others_flags | middle_flag) == 0)
        .map(|(idx, _)| WORD_LIST[idx].as_str())
        .collect()
}
