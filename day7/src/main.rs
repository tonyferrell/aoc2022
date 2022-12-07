mod aoc_file;
mod file_tree;
mod new_tree;
mod terminal_parser;

use std::{cell::RefCell, rc::Rc};

use new_tree::*;
use terminal_parser::*;

fn main() {
    println!("Hello, world!");
}

// fn load_file(filename: &str) -> Rc<TreeNode> {
//     let lines = aoc_file::read_lines(filename).unwrap();
//     let commands = lines.iter().map(|x| x.parse::<Terminal>().unwrap());

//     process_commands(commands.into_iter())
// }

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
                            None => DirectoryNode::new(Some(Rc::clone(&current_location)), into.to_owned())
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
                TermCommand::Ls => todo!(),
            },
            Terminal::LsResult(ls_result) => match ls_result {
                LsResult::FileContents(_) => todo!(),
                LsResult::Directory(_) => todo!(),
            },
        }
    }

    Ok(Rc::clone(&root))
}

#[test]
fn process_move_root_test() {
    // let root =
    //     process_commands(vec![Terminal::Command(TermCommand::Cd(ChangeDir::Root))].into_iter());
    // assert_eq!(
    //     *root,
    //     TreeNode::Directory(RefCell::new(DirectoryData {
    //         parent: None,
    //         name: "/".to_owned(),
    //         total_size: 0,
    //         children: vec![]
    //     }))
    // );
}

#[test]
fn file_children_test() {
    // let root = process_commands(
    //     vec![
    //         Terminal::Command(TermCommand::Cd(ChangeDir::Root)),
    //         Terminal::LsResult(LsResult::FileContents(terminal_parser::FileData {
    //             size: 14,
    //             filename: "unk.png".to_owned(),
    //         })),
    //     ]
    //     .into_iter(),
    // );

    // assert_eq!(
    //     *root,
    //     TreeNode::Directory(RefCell::new(DirectoryData {
    //         parent: None,
    //         name: "/".to_owned(),
    //         total_size: 14,
    //         children: vec![TreeNode::new_file(
    //             Rc::downgrade(&root),
    //             "unk.png".to_owned(),
    //             14
    //         )]
    //     }))
    // );
}

#[test]
fn read_file_test() {
    // let root = load_file("./test.txt");
    // assert_eq!(root.get_size(), 4);
}
