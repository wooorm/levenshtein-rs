extern crate levenshtein;
use levenshtein::levenshtein;

fn main() {
    print!("{}\n", levenshtein("kitten", "sitting"));
}
