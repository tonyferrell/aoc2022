use std::{
    cmp::Ordering,
    convert::identity,
    env,
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

use itertools::Itertools;

#[derive(PartialEq, Debug)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for RockPaperScissors {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl PartialOrd for RockPaperScissors {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let loss = Some(std::cmp::Ordering::Less);
        let win = Some(std::cmp::Ordering::Greater);
        match (self, other) {
            (Self::Rock, Self::Paper) => loss,
            (Self::Paper, Self::Scissors) => loss,
            (Self::Scissors, Self::Rock) => loss,
            (Self::Rock, Self::Scissors) => win,
            (Self::Paper, Self::Rock) => win,
            (Self::Scissors, Self::Paper) => win,
            (_, _) => Some(std::cmp::Ordering::Equal),
        }
    }
}

fn main() {
    let filepath = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Unable to parse file path!");
            return;
        }
    };

    let score = score_tournament_1(&filepath);
    println!("Final Score: {}", score);
}

fn read_lines(filename: &str) -> Option<Vec<String>> {
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines().map(|x| x.unwrap());

    Some(lines.collect::<Vec<_>>())
}

fn get_entries(filename: &str) -> Vec<(String, String)> {
    let content = read_lines(filename).unwrap();
    content
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|line_split| line_split.to_owned())
                .take(2)
                .collect_tuple()
        })
        .filter_map(identity)
        .collect()
}

fn score_tournament_1(filename: &str) -> i32 {
    let game_lines = get_entries(filename);
    game_lines
        .iter()
        .map(|game| score_game_1((&game.0, &game.1)))
        .sum()
}

fn score_game_1<T: AsRef<str>>(scores: (T, T)) -> i32 {
    let (theirs, mine): (RockPaperScissors, RockPaperScissors) = (
        scores.0.as_ref().parse().unwrap(),
        scores.1.as_ref().parse().unwrap(),
    );

    let game_score = match &mine.partial_cmp(&theirs).unwrap() {
        Ordering::Greater => 6,
        Ordering::Less => 0,
        Ordering::Equal => 3,
    };

    let my_play_score = match &mine {
        RockPaperScissors::Rock => 1,
        RockPaperScissors::Paper => 2,
        RockPaperScissors::Scissors => 3,
    };

    game_score + my_play_score
}

/*
For example, suppose you were given the following strategy guide:

A Y
B X
C Z
This strategy guide predicts and recommends the following:

In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6). */
#[test]
fn score_function_test() {
    assert_eq!(score_game_1(("A", "Y")), 8);
    assert_eq!(score_game_1(("B", "X")), 1);
    assert_eq!(score_game_1(("C", "Z")), 6);
}

#[test]
fn score_tournament_test() {
    assert_eq!(score_tournament_1("./test2.txt"), 15);
}

#[test]
fn parse_lines_test() {
    let line_tuples = get_entries("./test.txt");
    assert_eq!(
        line_tuples.first().unwrap(),
        &("C".to_owned(), "X".to_owned())
    );
    assert_eq!(
        line_tuples.iter().nth(1).unwrap(),
        &("B".to_owned(), "Y".to_owned())
    );
    assert_eq!(
        line_tuples.iter().nth(2).unwrap(),
        &("C".to_owned(), "Z".to_owned())
    );
    assert_eq!(
        line_tuples.iter().nth(3).unwrap(),
        &("C".to_owned(), "Z".to_owned())
    );
}

#[test]
fn rock_paper_scissors_ordering_test() {
    assert!(RockPaperScissors::Rock > RockPaperScissors::Scissors);
    assert!(RockPaperScissors::Paper > RockPaperScissors::Rock);
    assert!(RockPaperScissors::Scissors > RockPaperScissors::Paper);
}

#[test]
fn parse_enum_test() {
    assert_eq!(
        "A".parse::<RockPaperScissors>(),
        Ok(RockPaperScissors::Rock)
    );
    assert_eq!(
        "X".parse::<RockPaperScissors>(),
        Ok(RockPaperScissors::Rock)
    );
    assert_eq!(
        "B".parse::<RockPaperScissors>(),
        Ok(RockPaperScissors::Paper)
    );
    assert_eq!(
        "Y".parse::<RockPaperScissors>(),
        Ok(RockPaperScissors::Paper)
    );
    assert_eq!(
        "C".parse::<RockPaperScissors>(),
        Ok(RockPaperScissors::Scissors)
    );
    assert_eq!(
        "Z".parse::<RockPaperScissors>(),
        Ok(RockPaperScissors::Scissors)
    );
}
