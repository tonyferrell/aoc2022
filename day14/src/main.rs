mod aoc_file;
mod map;
mod matrix;
mod point;

use aoc_file::read_lines;
use itertools::Itertools;
use map::*;
use point::*;
use SandState::*;

enum SandState {
    Moved(Point),
    Stopped(Point),
    Escaped,
}

fn main() {
    let map: Map = parse_file_to_structure_definitions(&aoc_file::get_file_param())
        .unwrap()
        .into();
    println!("{}", map.data);
    play_sand_game(map);
}

fn parse_file_to_structure_definitions(filename: &str) -> Result<MapSpec, ()> {
    let lines = read_lines(filename).ok_or(())?;
    let mut max_x = 0;
    let mut max_y = 0;

    let rock_formations = lines
        .iter()
        .map(|line| {
            RockFormation(
                line.split(" -> ")
                    .map(|pairs| {
                        let (x, y) = pairs
                            .split(",")
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect_tuple()
                            .unwrap();
                        max_x = max_x.max(x);
                        max_y = max_y.max(y);

                        Point(x, y)
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    // Leave room for falling sand? Not sure if the edge of the map should be important here.
    let width = max_x + 10;
    let height = max_y + 1;

    Ok(MapSpec {
        width,
        height,
        rock_formations,
    })
}

fn tick(position: Point, map: &Map) -> SandState {
    let potenial_moves = [
        position.straight_down(),
        position.down_left(),
        position.down_right(),
    ];

    for the_move in potenial_moves {
        if the_move.1 >= map.data.height {
            return Escaped;
        } else if let MapCell::Air = map[&the_move.into()] {
            return Moved(the_move);
        }
    }

    Stopped(position)
}
fn play_sand_game(mut map: Map) -> Map {
    let mut counter = 0;
    loop {
        // New little grain of sand!
        let mut position = Point(500, 0);
        counter += 1;

        loop {
            match tick(position, &map) {
                Moved(pos) => {
                    position = pos;
                }
                Stopped(position) => {
                    map[position] = MapCell::Sand;
                    println!("{}", map.data);
                    break;
                }
                Escaped => {
                    println!("Went for {}", counter-1);
                    return map;
                }
            }
        }
    }
}
#[test]
fn test_map_parse_from_spec() {
    let map: Map = parse_file_to_structure_definitions("./test.txt")
        .expect("file should be readable")
        .into();
}

#[test]
fn parse_file_structures_test() {
    let MapSpec {
        width,
        height,
        rock_formations,
    } = parse_file_to_structure_definitions("./test.txt").expect("file should be readable");

    assert_eq!(width, 1006);
    assert_eq!(height, 18);
    assert_eq!(rock_formations.len(), 2);
    assert_eq!(
        rock_formations[0].0,
        vec![Point(498, 4), Point(498, 6), Point(496, 6)]
    );

    assert_eq!(
        rock_formations[1].0,
        vec![Point(503, 4), Point(502, 4), Point(502, 9), Point(494, 9)]
    );
}

#[test]
fn point_ordering_test() {
    let one = Point(100, 1);
    let two = Point(100, 2);

    assert!(one < two);
    assert!(two > one);

    let one = Point(100, 0);
    let two = Point(101, 0);

    assert!(one < two);
    assert!(two > one);

    assert_eq!(one.max(two.clone()), two);
}

#[test]
fn line_generation_test() {
    /* 498,4 -> 498,6 -> 496,6 */
    assert_eq!(
        Point::line_expand(&Point(498, 4)..&Point(498, 6)).collect::<Vec<_>>(),
        vec![Point(498, 4), Point(498, 5), Point(498, 6)]
    );

    assert_eq!(
        Point::line_expand(&Point(498, 6)..&Point(496, 6)).collect::<Vec<_>>(),
        vec![Point(496, 6), Point(497, 6), Point(498, 6)]
    );
    // 503,4 -> 502,4 -> 502,9 -> 494,9
    assert_eq!(
        Point::line_expand(&Point(503, 4)..&Point(502, 4)).collect::<Vec<_>>(),
        vec![Point(502, 4), Point(503, 4),]
    );
    assert_eq!(
        Point::line_expand(&Point(502, 4)..&Point(502, 9)).collect::<Vec<_>>(),
        vec![
            Point(502, 4),
            Point(502, 5),
            Point(502, 6),
            Point(502, 7),
            Point(502, 8),
            Point(502, 9),
        ]
    );
    assert_eq!(
        Point::line_expand(&Point(502, 9)..&Point(494, 9)).collect::<Vec<_>>(),
        vec![
            Point(494, 9),
            Point(495, 9),
            Point(496, 9),
            Point(497, 9),
            Point(498, 9),
            Point(499, 9),
            Point(500, 9),
            Point(501, 9),
            Point(502, 9),
        ]
    );
}
