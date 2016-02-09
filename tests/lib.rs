extern crate levenshtein;
use levenshtein::levenshtein;

#[test]
fn empty_left() {
    assert_eq!(levenshtein("", "a"), 1);
}

#[test]
fn empty_right() {
    assert_eq!(levenshtein("a", ""), 1);
}

#[test]
fn empty_both() {
    assert_eq!(levenshtein("", ""), 0);
}

#[test]
fn equal_long() {
    assert_eq!(levenshtein("levenshtein", "levenshtein"), 0);
}

#[test]
fn case_sensitive() {
    assert_eq!(levenshtein("DwAyNE", "DUANE"), 2);
    assert_eq!(levenshtein("dwayne", "DuAnE"), 5);
}

#[test]
fn ordering() {
    assert_eq!(levenshtein("aarrgh", "aargh"), 1);
    assert_eq!(levenshtein("aargh", "aarrgh"), 1);
}

#[test]
fn should_work() {
    assert_eq!(levenshtein("sitting", "kitten"), 3);
    assert_eq!(levenshtein("gumbo", "gambol"), 2);
    assert_eq!(levenshtein("saturday", "sunday"), 3);
    assert_eq!(levenshtein("a", "b"), 1);
    assert_eq!(levenshtein("ab", "ac"), 1);
    assert_eq!(levenshtein("ac", "bc"), 1);
    assert_eq!(levenshtein("abc", "axc"), 1);
    assert_eq!(levenshtein("xabxcdxxefxgx", "1ab2cd34ef5g6"), 6);
    assert_eq!(levenshtein("xabxcdxxefxgx", "abcdefg"), 6);
    assert_eq!(levenshtein("javawasneat", "scalaisgreat"), 7);
    assert_eq!(levenshtein("example", "samples"), 3);
    assert_eq!(levenshtein("sturgeon", "urgently"), 6);
    assert_eq!(levenshtein("levenshtein", "frankenstein"), 6);
    assert_eq!(levenshtein("distance", "difference"), 5);
    assert_eq!(levenshtein("kitten", "sitting"), 3);
    assert_eq!(levenshtein("Tier", "Tor"), 2);
}

#[test]
fn unicode() {
    assert_eq!(levenshtein("😄", "😦"), 1);
    assert_eq!(levenshtein("😘", "😘"), 0);
}
