mod aoc_file;
mod file_tree;
mod new_tree;
mod terminal_parser;

use std::{cell::RefCell, rc::Rc};

use new_tree::*;
use terminal_parser::*;

fn main() {
    let root = load_file(&aoc_file::get_file_param()).unwrap();
    let directories = flatten_directories(Rc::clone(&root));
    let sum = directories
        .iter()
        .filter_map(|(_, size)| if size < &100000 { Some(size) } else { None })
        .sum::<usize>();

    let needed = 30000000 - (70000000 - root.borrow().size);

    let mut smallest = directories
        .iter()
        .filter(|(_, size)| size > &needed)
        .collect::<Vec<_>>();

    smallest.sort_by_key(|(_, size)| size);
    let solution = dbg!(*smallest.first().unwrap());

    println!("Sum1: {}", sum);
    println!("We need: {}, and the smallest: {:?}", needed, solution)
}

fn load_file(filename: &str) -> Result<Rc<RefCell<DirectoryNode>>, String> {
    let lines = aoc_file::read_lines(filename).unwrap();
    let commands = lines.iter().map(|x| x.parse::<Terminal>().unwrap());

    process_commands(commands.into_iter())
}

fn process_commands<T: Iterator<Item = Terminal>>(
    lines: T,
) -> Result<Rc<RefCell<DirectoryNode>>, String> {
    let root = DirectoryNode::new(None, "/".to_owned());
    let mut current_location = Rc::clone(&root);

    for (i, line) in lines.enumerate() {
        match line {
            Terminal::Noop => (),
            Terminal::Command(command) => match command {
                TermCommand::Cd(cd) => match cd {
                    ChangeDir::In(into) => {
                        let child = current_location.borrow().get_child(&into);
                        current_location = match child {
                            Some(child) => child,
                            None => todo!(),
                        }
                    }
                    ChangeDir::Out => {
                        let parent = match &(*current_location).borrow().parent {
                            Some(x) => Rc::clone(x),
                            None => {
                                return Err(format!("Line {}, attempted to move up too far.", i));
                            }
                        };

                        current_location = parent;
                    }
                    ChangeDir::Root => current_location = Rc::clone(&root),
                },
                TermCommand::Ls => (),
            },
            Terminal::LsResult(ls_result) => match ls_result {
                LsResult::FileContents(file) => current_location
                    .borrow_mut()
                    .add_child_file(file.filename, file.size),
                LsResult::Directory(name) => {
                    let new_node =
                        DirectoryNode::new(Some(Rc::clone(&current_location)), name.to_owned());
                    current_location.borrow_mut().add_child_dir(new_node);
                }
            },
        }
    }

    Ok(Rc::clone(&root))
}

fn flatten_directories(root: Rc<RefCell<DirectoryNode>>) -> Vec<(String, usize)> {
    let mut to_walk = vec![root];
    let mut dirs: Vec<(String, usize)> = vec![];

    while let Some(next) = to_walk.pop() {
        let next = next.borrow();
        dirs.push((next.name.to_owned(), next.size));
        for child in &next.directory_children {
            to_walk.push(Rc::clone(child));
        }
    }

    dirs
}

#[test]
fn file_children_test() {
    let root = process_commands(
        vec![
            Terminal::Command(TermCommand::Cd(ChangeDir::Root)),
            Terminal::LsResult(LsResult::FileContents(terminal_parser::FileData {
                size: 14,
                filename: "unk.png".to_owned(),
            })),
        ]
        .into_iter(),
    )
    .unwrap();

    assert_eq!(
        *root.borrow(),
        DirectoryNode {
            parent: None,
            name: "/".to_owned(),
            size: 14,
            directory_children: vec![],
            file_children: vec!["unk.png".to_owned()].into_iter().collect(),
        }
    );
}

#[test]
fn read_file_test() {
    let root = load_file("./test.txt").unwrap();
    assert_eq!(root.borrow().size, 48381165);

    assert_eq!(
        flatten_directories(root)
            .iter()
            .filter_map(|(_, size)| if size < &100000 { Some(size) } else { None })
            .sum::<usize>(),
        95437
    );
}

#[test]
fn small_thing_test() {
    let root = load_file("./test.txt").unwrap();
    let needed = 30000000 - (70000000 - root.borrow().size);

    let directories = flatten_directories(Rc::clone(&root));
    let mut smallest = directories
        .iter()
        .filter(|(_, size)| size > &needed)
        .collect::<Vec<_>>();

    smallest.sort_by_key(|(_, size)| size);
    let solution = dbg!(*smallest.first().unwrap());
    assert_eq!(solution.1, 24933642);
}
