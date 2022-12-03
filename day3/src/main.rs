use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    println!("Priority Sum: {}", score_file("./input.txt"));
    println!("Badge Score: {}", badge_score_file("./input.txt"));
}

fn score_line(line: &str) -> i32 {
    let middle = line.len() / 2;
    let left: HashSet<char> = line.chars().take(middle).collect();
    let right: HashSet<char> = line.chars().skip(middle).collect();

    let both = left.intersection(&right);
    both.into_iter().map(|x| get_priority(*x)).sum()
}

fn get_badge_scores<T: AsRef<str>>(elves: &[T]) -> i32 {
    elves
        .chunks(3)
        .map(|chunk| {
            *chunk
                .iter()
                .map(|x| (*x.as_ref()).chars().collect::<HashSet<_>>())
                .reduce(|acc, next| acc.intersection(&next).into_iter().map(|x| *x).collect())
                .unwrap()
                .iter()
                .next()
                .unwrap()
        })
        .map(get_priority)
        .sum()
}

fn get_priority(c: char) -> i32 {
    let c = c as i32;
    if c <= 90 {
        // Assume uppercase letter
        c - 38
    } else {
        c - 96
    }
}

fn score_file(filename: &str) -> i32 {
    read_lines(filename)
        .unwrap()
        .iter()
        .map(|x| score_line(x))
        .sum()
}

fn badge_score_file(filename: &str) -> i32 {
    get_badge_scores(&read_lines(filename).unwrap())
}

fn read_lines(filename: &str) -> Option<Vec<String>> {
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines().map(|x| x.unwrap());

    Some(lines.collect::<Vec<_>>())
}

/*
For example, suppose you have the following list of contents from six rucksacks:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is uppercase L.
The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
The fourth rucksack's compartments only share item type v.
The fifth rucksack's compartments only share item type t.
The sixth rucksack's compartments only share item type s.
To help prioritize item rearrangement, every item type can be converted to a priority:

Lowercase item types a through z have priorities 1 through 26.
Uppercase item types A through Z have priorities 27 through 52.
In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157. */
#[test]
fn iterate_alphabet_test() {
    let lower_case_letters = "abcdefghijklmnopqrstuvwxyz".to_owned();
    for (i, c) in lower_case_letters.chars().enumerate() {
        assert_eq!(get_priority(c), (i + 1) as i32);
    }

    let upper_case_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned();
    for (i, c) in upper_case_letters.chars().enumerate() {
        assert_eq!(get_priority(c), (i + 27) as i32);
    }
}

#[test]
fn test_simple_score_line() {
    assert_eq!(score_line("abcabd"), 3);
    assert_eq!(score_line("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
    assert_eq!(score_line("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 38);
    assert_eq!(score_line("PmmdzqPrVvPwwTWBwg"), 42);
    assert_eq!(score_line("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 22);
    assert_eq!(score_line("ttgJtRGJQctTZtZT"), 20);
    assert_eq!(score_line("CrZsJsPPZsGzwwsLwLmpwMDw"), 19);
}

#[test]
fn test_score_input() {
    assert_eq!(score_file("./test.txt"), 157)
}

#[test]
fn test_simple_chunks() {
    let group1 = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
    ];

    assert_eq!(get_badge_scores(&group1), 18);

    let group2 = vec![
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    assert_eq!(get_badge_scores(&group2), 52)
}

#[test]
fn test_badge_scores() {
    assert_eq!(badge_score_file("./test.txt"), 70);
}
