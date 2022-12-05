use std::fmt::Debug;

mod aoc_file;

fn main() {
    read_input_file(&aoc_file::get_file_param());
}

fn read_input_file(filename: &str) -> (Game, Vec<Move>) {
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

    for line in board.iter().rev() {
        let parse_board = line
            .chunks(4)
            .map(|col_val| col_val[1])
            .enumerate()
            .filter(|(_, v)| !v.is_whitespace());

        for (column, value) in parse_board {
            game_board.add(value, column + 1);
        }
    }

    let instructions: Vec<_> = instructions
        .iter()
        .flat_map(|line| {
            let mut tokens = line.split_whitespace();
            let count: usize = tokens.nth(1).unwrap().parse().unwrap();
            let from: usize = tokens.nth(1).unwrap().parse().unwrap();
            let to: usize = tokens.nth(1).unwrap().parse().unwrap();

            (1..count + 1)
                .map(|_| Move::new(from, to))
                .collect::<Vec<_>>()
        })
        .collect();

    (game_board, instructions)
}

#[derive(PartialEq, Eq, Debug)]
struct Move {
    from: usize,
    to: usize,
}

impl Move {
    fn new(from: usize, to: usize) -> Move {
        Move { from, to }
    }
}
#[non_exhaustive]
#[derive(Debug)]
struct Game {
    stacks: Vec<Vec<char>>,
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

    fn make_move(&mut self, the_move: &Move) -> Result<(), String> {
        let source = the_move.from;
        let dest = the_move.to;

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

    fn get_stack_top(&self) -> String {
        let mut top = String::new();
        for stack in &self.stacks {
            match stack.last() {
                Some(val) => top.push(*val),
                None => (),
            };
        }
        top
    }
}

fn play_game(filename: &str) -> Game {
    let (mut game, instructions) = read_input_file(filename);

    dbg!(&game);

    for m in instructions {
        game.make_move(&m).unwrap();
        dbg!(&m, &game);
    }

    game
}

#[test]
fn parse_board_test() {
    let (game, instructions) = read_input_file("./test1.txt");
    // println!("{:#?}", game);

    // Write some real asserts here
    assert_eq!(game.stacks[0], vec!['Z', 'N']);
    assert_eq!(game.stacks[1], vec!['M', 'C', 'D']);
    assert_eq!(game.stacks[2], vec!['P']);

    assert_eq!(
        instructions,
        vec![
            Move::new(2, 1),
            Move::new(1, 3),
            Move::new(1, 3),
            Move::new(1, 3),
            Move::new(2, 1),
            Move::new(2, 1),
            Move::new(1, 2),
        ]
    )
}

#[test]
fn parse_board_pop_test() {
    let (mut game, _) = read_input_file("./test1.txt");
    // println!("{:#?}", game);

    dbg!(&game);
    assert_eq!(game.get_stack_top(), "NDP");

    assert_eq!(game.stacks[0], vec!['Z', 'N']);
    assert_eq!(game.stacks[1], vec!['M', 'C', 'D']);
    assert_eq!(game.stacks[2], vec!['P']);

    game.make_move(&Move::new(1, 2)).unwrap();
    dbg!(&game);
    assert_eq!(game.stacks[1], vec!['M', 'C', 'D', 'N' ]);
    assert_eq!(game.get_stack_top(), "ZNP");
    game.make_move(&Move::new(3,1)).unwrap();
    dbg!(&game);
}

#[test]
fn get_stack_top_test() {
    let g = Game {
        stacks: vec![vec!['A', 'B'], vec!['C', 'D'], vec!['E', 'F', 'G']],
    };

    assert_eq!(dbg!(g).get_stack_top(), "BDG")
}

#[test]
fn play_game_test() {
    let game = play_game("./test1.txt");

    assert_eq!(game.get_stack_top(), "CMZ");
}
