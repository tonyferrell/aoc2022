use std::collections::BinaryHeap;
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};

struct ElfMaker<'a, T: AsRef<str>> {
    next_id: u32,
    remainder: Option<&'a [T]>,
}

#[derive(Debug, Eq)]
struct Elf {
    number: u32,
    calories: u32,
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.calories.cmp(&other.calories)
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.calories.partial_cmp(&other.calories)
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.calories == other.calories
    }
}

trait LineGrouper<T: AsRef<str>> {
    fn get_next_group(&mut self) -> Option<&[T]>;
}

impl<'a, T: AsRef<str>> LineGrouper<T> for ElfMaker<'a, T> {
    fn get_next_group(&mut self) -> Option<&[T]> {
        if let Some(ref mut remainder) = self.remainder {
            let end = remainder.into_iter().position(|x| x.as_ref().is_empty())?;
            let rest = &remainder[..end];
            *remainder = &remainder[end + 1..];

            Some(rest)
        } else {
            None
        }
    }
}

impl<'a, T: AsRef<str>> ElfMaker<'a, T> {
    fn new(lines: &'a [T]) -> Self {
        ElfMaker {
            next_id: 0,
            remainder: Some(lines),
        }
    }
}

impl<'a, T: AsRef<str>> Iterator for ElfMaker<'a, T> {
    type Item = Elf;

    fn next(&mut self) -> Option<Self::Item> {
        let number = self.next_id;
        self.next_id += 1;

        let group = self.get_next_group()?;
        let calories = group
            .into_iter()
            .map(|x| x.as_ref().parse::<u32>().unwrap_or(0))
            .sum();

        let the_elf = Elf { number, calories };

        return Some(the_elf);
    }
}

fn main() {
    // // Read the file name
    let filepath = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Unable to parse file path!");
            return;
        }
    };

    let max = max_elf(&filepath);
    println!("Maximum Elf: {:?}", max.unwrap());

    let max3 = max_three_elves(&filepath).unwrap();
    println!("Maximum 3: {:?}", max3);
}

fn max_elf(filename: &str) -> Option<Elf> {
    let lines = read_lines(filename)?;
    let em = ElfMaker::new(&lines);

    em.max_by_key(|e| e.calories)
}

fn max_three_elves(filename: &str) -> Option<u32> {
    let lines = read_lines(filename)?;

    let em = ElfMaker::new(&lines);
    let mut data: BinaryHeap<_> = em.collect();
    // For no sane reason the iterator is not ordered, and drain_sorted and into_iter_sorted are not stabilized...
    // Some(data.drain_sorted().take(3).map(|x| {
    //     println!("{}",x.calories);
    //     x.calories
    // }).sum())
    Some(
        [data.pop(), data.pop(), data.pop()]
            .map(|x| {
                let internal = x.unwrap();
                println!("{}", internal.calories);
                internal.calories
            })
            .iter()
            .sum(),
    )
}

fn read_lines(filename: &str) -> Option<Vec<String>> {
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines().map(|x| x.unwrap());

    Some(lines.collect::<Vec<_>>())
}

#[test]
fn fake_one_elf_test() {
    let lines = vec!["1", "2", "3", ""];
    let em = ElfMaker::new(&lines);

    let binding = em.collect::<Vec<_>>();
    let elf = binding.first().expect("We expect there to be a first elf");

    assert_eq!(elf.number, 0);
    assert_eq!(elf.calories, 6, "First elf should have 6 calories");
}

#[test]
fn fake_multi_elf_test() {
    let lines = vec![
        "1", "2", "3", "", "2", "2", "2", "2", "", "3", "", "4", "4", "",
    ];
    let em = ElfMaker::new(&lines);

    let binding = em.collect::<Vec<_>>();
    let elf1 = binding.first().unwrap();

    assert_eq!(elf1.number, 0);
    assert_eq!(elf1.calories, 6);

    let elf2 = binding.iter().nth(1).unwrap();
    assert_eq!(elf2.number, 1);
    assert_eq!(elf2.calories, 8);

    let elf3 = binding.iter().nth(2).unwrap();
    assert_eq!(elf3.number, 2);
    assert_eq!(elf3.calories, 3);

    let elf4 = binding.iter().nth(3).unwrap();
    assert_eq!(elf4.number, 3);
    assert_eq!(elf4.calories, 8);
}

#[test]
fn file_read_test() {
    // let read_lines
    let lines = read_lines("./testinput.txt".as_ref());
    match lines {
        Some(lines) => {
            assert_eq!(lines.len(), 12, "line count");
        }
        None => {
            assert!(lines.is_some())
        }
    }
}

#[test]
fn elf_counts() {
    let lines = read_lines("./testinput.txt".as_ref()).unwrap();
    let em = ElfMaker::new(&lines);

    let elves: Vec<Elf> = em.collect();
    println!("{:?}", elves);

    assert_eq!(elves.len(), 4);
}

#[test]
fn file_max_elf_test() {
    let max = max_elf("./testinput.txt".as_ref()).unwrap();
    assert_eq!(max.number, 2, "number");
    assert_eq!(max.calories, 9, "calories");
}

#[test]
fn file_max_three_test() {
    let max3 = max_three_elves("./testinput.txt".as_ref()).unwrap();
    assert_eq!(max3, 17, "top3");
}
