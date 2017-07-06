extern crate levenshtein;
use levenshtein::levenshtein;

fn main() {
    println!("{}", levenshtein("kitten", "sitting"));
}
