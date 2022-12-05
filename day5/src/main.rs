use std::fmt::Debug;

mod aoc_file;

fn main() {
    read_input_file(&aoc_file::get_file_param());
}

fn read_input_file(filename: &str) -> Game {
    let lines = aoc_file::read_lines(filename).expect("please only give us files that exist.");
    let board: Vec<Vec<_>> = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        // Kind of janky, but take until we get the column definitions (which are the only numerals)
        .take_while(|x| x.get(1) != Some(&'1'))
        .collect();

    let instructions: Vec<_> = lines.iter().skip_while(|x| !x.is_empty()).skip(1).collect();

    // Game board with no columns, we'll handle that as we add them.
    let mut game_board = Game { stacks: vec![] };

    for line in board {
        let parse_board = line
            .chunks(4)
            .map(|col_val| col_val[1])
            .enumerate()
            .filter(|(_, v)| !v.is_whitespace());

        for (column, value) in parse_board {
            game_board.add(value, column + 1);
        }
    }

    println!("Instructions:");
    for instruction in instructions {
        println!("{}", instruction);
    }

    game_board
}

#[non_exhaustive]
struct Game {
    stacks: Vec<Vec<char>>,
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Game")
            .field("stacks", &self.stacks)
            .finish()
    }
}

impl Game {
    fn add(&mut self, val: char, col: usize) {
        while self.stacks.len() < col {
            self.stacks.push(vec![]);
        }

        // Get columns back to indexes.
        let col = col - 1;

        self.stacks
            .get_mut(col)
            .expect("adding failed because the column is missing")
            .push(val);
    }

    fn make_move(&mut self, source: usize, dest: usize) -> Result<(), String> {
        let size = self.stacks.len();
        if source < 1 || source > size {
            return Err(format!("Source range out of bounds: {}", source));
        } else if dest < 1 || dest > size {
            return Err(format!("Dest range out of bounds: {}", source));
        }

        let source = source - 1;
        let dest = dest - 1;
        let source_value = self
            .stacks
            .get_mut(source)
            .expect("destination column should exist")
            .pop()
            .expect("destination column should not be empty");
        self.stacks
            .get_mut(dest)
            .expect("source column should exist")
            .push(source_value);

        Ok(())
    }
}

#[test]
fn parse_board_test() {
    let game = read_input_file("./test1.txt");
    // println!("{:#?}", game);

    // Write some real asserts here
    assert_eq!(game.stacks[0], vec!['N', 'Z']);
    assert_eq!(game.stacks[1], vec!['D', 'C', 'M']);
    assert_eq!(game.stacks[2], vec!['P']);
}
