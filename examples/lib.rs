extern crate levenshtein;
use levenshtein::{closest, distance};

fn main() {
    println!("{}", distance("kitten", "sitting"));
    println!(
        "{}",
        closest("levenshtein", &["frankenstein", "einstein"]).unwrap()
    );
}
