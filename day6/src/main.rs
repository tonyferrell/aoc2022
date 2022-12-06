mod aoc_file;

use std::{collections::VecDeque, str::FromStr};

struct SlidingWindow {
    window_size: usize,
    window: VecDeque<char>,
    word_count: VecDeque<u8>,
}

impl SlidingWindow {
    fn new(window_size: usize) -> Self {
        Self {
            window_size,
            window: VecDeque::new(),
            word_count: dbg!((1..27).map(|_| 0).collect()),
        }
    }

    fn cycle_char(&mut self, next: char) -> u8 {
        assert!(self.window.len() <= self.window_size);

        let next = next.to_ascii_lowercase();

        if self.window.len() == self.window_size {
            let remove_char = self.window.pop_front().unwrap();
            self.word_count[remove_char as usize - 97] -= 1;
        }

        self.window.push_back(dbg!(next));
        dbg!(self.word_count.len());
        self.word_count[dbg!(next as usize - 97)] += 1;

        self.count_unique_chars()
    }

    fn count_unique_chars(&self) -> u8 {
        self.word_count
            .iter()
            .filter_map(|x| if *x == 0u8 { None } else { Some(1u8) })
            .sum()
    }
}

struct Signal {
    full_signal: String,
    start_offset: usize,
    start_of_message: usize,
}

impl FromStr for Signal {
    type Err = ();

    fn from_str(full_signal: &str) -> Result<Self, Self::Err> {
        let mut window = SlidingWindow::new(4);
        let start_offset = full_signal
            .chars()
            .take_while(|c| window.cycle_char(*c) < 4)
            .count()
            + 1;

        let mut window = SlidingWindow::new(14);
        let start_of_message = full_signal
            .chars()
            .take_while(|c| window.cycle_char(*c) < 14)
            .count()
            + 1;

        Ok(Signal {
            full_signal: full_signal.to_owned(),
            start_offset,
            start_of_message,
        })
    }
}

fn main() {
    let lines = aoc_file::read_lines(&aoc_file::get_file_param()).unwrap();
    for line in lines {
        let signal: Signal = line.parse().unwrap();
        println!("Start: {}", signal.start_offset);
        println!("Start of Message: {}", signal.start_of_message);
    }
}

#[test]
fn cycle_char_4_test() {
    let mut window = SlidingWindow::new(4);
    // Add the sequence "aabb" - which should give us a unique character count of 2.
    let letter_count = window.cycle_char('a');
    assert_eq!(letter_count, 1u8);

    let letter_count = window.cycle_char('a');
    assert_eq!(letter_count, 1u8);

    let letter_count = window.cycle_char('b');
    assert_eq!(letter_count, 2u8);

    let letter_count = window.cycle_char('b');
    assert_eq!(letter_count, 2u8);

    // Now, add acd - which should get us up to a unique character count of 4.
    let letter_count = window.cycle_char('a');
    assert_eq!(letter_count, 2u8);

    let letter_count = window.cycle_char('c');
    assert_eq!(letter_count, 3u8);

    let letter_count = window.cycle_char('d');
    assert_eq!(letter_count, 4u8);
}
/*
mjqjpqmgbljsphdztnvjfqwrcgsmlb - In this case, your subroutine should report the value 7, because the first start-of-packet marker is complete after 7 characters have been processed.
bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11
 */
#[test]
fn signal_start_test_1() {
    let s = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".parse::<Signal>().unwrap();
    assert_eq!(s.start_offset, 7);
    assert_eq!(s.start_of_message, 19);
}

#[test]
fn signal_start_test_2() {
    let s = "bvwbjplbgvbhsrlpgdmjqwftvncz".parse::<Signal>().unwrap();
    assert_eq!(s.start_offset, 5);
    assert_eq!(s.start_of_message, 23);
}

#[test]
fn signal_start_test_3() {
    let s = "nppdvjthqldpwncqszvftbrmjlhg".parse::<Signal>().unwrap();
    assert_eq!(s.start_offset, 6);
    assert_eq!(s.start_of_message, 23);
}

#[test]
fn signal_start_test_4() {
    let s = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
        .parse::<Signal>()
        .unwrap();
    assert_eq!(s.start_offset, 10);
    assert_eq!(s.start_of_message, 29);
}

#[test]
fn signal_start_test_5() {
    let s = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
        .parse::<Signal>()
        .unwrap();
    assert_eq!(s.start_offset, 11);
    assert_eq!(s.start_of_message, 26);
}
