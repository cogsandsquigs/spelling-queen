mod cli;
mod solver;

fn main() {
    let middle = 'i';
    let others = ['a', 'm', 'y', 'd', 't', 'e'];

    println!("{:?}", solver::get_possible_words(middle, others));
}
