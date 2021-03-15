extern crate levenshtein;
use levenshtein::{closest, distance};

#[test]
fn test_distance_empty_left() {
    assert_eq!(distance("", "a"), 1);
}

#[test]
fn test_distance_empty_right() {
    assert_eq!(distance("a", ""), 1);
}

#[test]
fn test_distance_empty_both() {
    assert_eq!(distance("", ""), 0);
}

#[test]
fn test_distance_equal_long() {
    assert_eq!(distance("levenshtein", "levenshtein"), 0);
}

#[test]
fn test_distance_case_sensitive() {
    assert_eq!(distance("DwAyNE", "DUANE"), 2);
    assert_eq!(distance("dwayne", "DuAnE"), 5);
}

#[test]
fn test_distance_ordering() {
    assert_eq!(distance("aarrgh", "aargh"), 1);
    assert_eq!(distance("aargh", "aarrgh"), 1);
}

#[test]
fn test_distance_should_work() {
    assert_eq!(distance("sitting", "kitten"), 3);
    assert_eq!(distance("gumbo", "gambol"), 2);
    assert_eq!(distance("saturday", "sunday"), 3);
    assert_eq!(distance("a", "b"), 1);
    assert_eq!(distance("ab", "ac"), 1);
    assert_eq!(distance("ac", "bc"), 1);
    assert_eq!(distance("abc", "axc"), 1);
    assert_eq!(distance("xabxcdxxefxgx", "1ab2cd34ef5g6"), 6);
    assert_eq!(distance("xabxcdxxefxgx", "abcdefg"), 6);
    assert_eq!(distance("javawasneat", "scalaisgreat"), 7);
    assert_eq!(distance("example", "samples"), 3);
    assert_eq!(distance("sturgeon", "urgently"), 6);
    assert_eq!(distance("levenshtein", "frankenstein"), 6);
    assert_eq!(distance("distance", "difference"), 5);
    assert_eq!(distance("kitten", "sitting"), 3);
    assert_eq!(distance("Tier", "Tor"), 2);
}

#[test]
fn test_distance_unicode() {
    assert_eq!(distance("😄", "😦"), 1);
    assert_eq!(distance("😘", "😘"), 0);
    assert_eq!(distance("نهصد", "نه‌صد"), 1);
    assert_eq!(distance("نهصد", "نه صد"), 1);
}

#[test]
fn test_closest_empties() {
    assert_eq!(closest("a", &[]), None);
    assert_eq!(closest("a", &["", "", "a", ""]), Some("a"));
    assert_eq!(closest("a", &["a", ""]), Some("a"));
}

#[test]
fn test_closest_should_work() {
    assert_eq!(closest("example", &["stamp", "sample"]), Some("sample"));
    assert_eq!(
        closest("levenshtein", &["frankenstein", "einstein"]),
        Some("einstein")
    );
    assert_eq!(closest("sitting", &["kitten", "kettle"]), Some("kitten"));
}

#[test]
fn test_closest_unicode() {
    assert_eq!(closest("نهصد", &["نه صد", "نهصد"]), Some("نهصد"));
    assert_eq!(closest("نهصد", &["نه صد", "نه‌صد"]), Some("نه صد"));
}
