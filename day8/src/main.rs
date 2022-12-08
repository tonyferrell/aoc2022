mod aoc_file;

fn main() {
    println!("Hello, world!");
}

fn load_file(filename: &str) -> Vec<Vec<usize>> {
    let lines = aoc_file::read_lines(filename).unwrap();

    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

#[test]
fn load_file_test() {
    let file = load_file("./test.txt");
    assert_eq!(file, vec!{
        vec![3,0,3,7,3],
        vec![2,5,5,1,2],
        vec![6,5,3,3,2],
        vec![3,3,5,4,9],
        vec![3,5,3,9,0],
    });
}