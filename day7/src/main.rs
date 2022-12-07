mod aoc_file;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Terminal {
    Command(TermCommand),
    LsResult(LsResult),
    Noop,
}

#[derive(Debug, PartialEq)]
enum TermCommand {
    Cd(ChangeDir),
    Ls,
}

#[derive(Debug, PartialEq)]
enum ChangeDir {
    In(String),
    Out,
    Root,
}

impl FromStr for ChangeDir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ".." => Ok(Self::Out),
            "/" => Ok(Self::Root),
            other => Ok(Self::In(other.to_owned())),
        }
    }
}

#[derive(Debug, PartialEq)]
enum LsResult {
    FileContents(FileData),
    Directory(String),
}

#[derive(Debug, PartialEq)]
struct FileData {
    size: usize,
    filename: String,
}

impl FromStr for TermCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c: Vec<_> = s.split_whitespace().collect();
        match c[1..] {
            ["cd", target] => Ok(Self::Cd(target.parse().unwrap())),
            ["ls"] => Ok(Self::Ls),
            _ => Err(format!("Unknown command {}", s)),
        }
    }
}

impl FromStr for LsResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line: Vec<_> = s.split_whitespace().collect();
        match line[0] {
            "dir" => Ok(Self::Directory(line[1].to_owned())),
            other => match other.parse::<usize>() {
                Ok(size) => Ok(Self::FileContents(FileData {
                    size,
                    filename: line[1].to_owned(),
                })),
                Err(_) => Err(format!("Unable to parse {}", other)),
            },
        }
    }
}
impl FromStr for Terminal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Self::Noop);
        } else if &s[0..1] == "$" {
            return Ok(Terminal::Command(s.parse()?));
        } else {
            return Ok(Terminal::LsResult(s.parse()?));
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn parse_ls_test() {
    let t: Terminal = "$ ls".parse().unwrap();
    assert_eq!(t, Terminal::Command(TermCommand::Ls));
}

#[test]
fn parse_cd_root_test() {
    let t: Terminal = "$ cd /".parse().unwrap();
    assert_eq!(t, Terminal::Command(TermCommand::Cd(ChangeDir::Root)));
}

#[test]
fn parse_cd_in_test() {
    let t: Terminal = "$ cd somedir".parse().unwrap();
    assert_eq!(
        t,
        Terminal::Command(TermCommand::Cd(ChangeDir::In("somedir".to_owned())))
    );
}

#[test]
fn parse_cd_up_test() {
    let t: Terminal = "$ cd ..".parse().unwrap();
    assert_eq!(t, Terminal::Command(TermCommand::Cd(ChangeDir::Out)));
}

#[test]
fn parse_filedata_test() {
    let t: Terminal = "8128312 file.som".parse().unwrap();
    assert_eq!(
        t,
        Terminal::LsResult(LsResult::FileContents(FileData {
            size: 8128312,
            filename: "file.som".to_owned()
        }))
    );
}

#[test]
fn parse_directory_data() {
    let t: Terminal = "dir somedir".parse().unwrap();
    assert_eq!(
        t,
        Terminal::LsResult(LsResult::Directory("somedir".to_owned()))
    );
}
