use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

pub fn get_file_param() -> String {
    env::args().nth(1).unwrap()
}

pub fn read_lines(filename: &str) -> Option<Vec<String>> {
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines().map(|x| x.unwrap());

    Some(lines.collect::<Vec<_>>())
}
