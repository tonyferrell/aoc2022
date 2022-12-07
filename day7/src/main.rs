mod aoc_file;
mod file_tree;
mod terminal_parser;

use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use file_tree::*;
use terminal_parser::*;

fn main() {
    println!("Hello, world!");
}

fn load_file(filename: &str) -> Rc<TreeNode> {
    let lines = aoc_file::read_lines(filename).unwrap();
    let commands = lines.iter().map(|x| x.parse::<Terminal>().unwrap());

    process_commands(commands.into_iter())
}

fn process_commands<T: Iterator<Item = Terminal>>(c: T) -> Rc<TreeNode> {
    let root = TreeNode::new_dir(None, "/".to_owned());
    let mut current_location: Rc<TreeNode> = Rc::clone(&root);
    for command in c {
        match command {
            Terminal::Command(command) => match command {
                TermCommand::Cd(cd_command) => match cd_command {
                    ChangeDir::In(name) => {
                        let mut children = match &*current_location {
                            TreeNode::Directory(dir) => &dir.borrow_mut().children,

                            _ => todo!(),
                        };

                        let val = children.iter().find(|&x|match &*x.as_ref() {
                            TreeNode::Directory(data) => data.borrow().name == name,
                            TreeNode::File(_) => todo!(),
                        }) ;

                    },
                    ChangeDir::Out => {
                        let new_location = match &*current_location {
                            TreeNode::Directory(dir) => {
                                Rc::clone(&dir.borrow().parent.as_ref().unwrap().upgrade().unwrap())
                            }
                            TreeNode::File(_) => todo!(),
                        };

                        current_location = new_location;
                    }
                    ChangeDir::Root => current_location = Rc::clone(&root),
                },
                TermCommand::Ls => (),
            },
            Terminal::LsResult(res) => match res {
                LsResult::FileContents(file) => match current_location.borrow() {
                    TreeNode::Directory(dir) => {
                        dir.borrow_mut().add_child(TreeNode::new_file(
                            Rc::downgrade(&current_location),
                            file.filename,
                            file.size,
                        ));
                    }
                    _ => todo!(),
                },
                LsResult::Directory(name) => {
                    let new_dir = TreeNode::new_dir(Some(Rc::downgrade(&current_location)), name);

                    match &*current_location {
                        TreeNode::Directory(dir) => {
                            dir.borrow_mut().add_child(new_dir);
                        }
                        TreeNode::File(_) => todo!(),
                    };
                }
            },
            Terminal::Noop => (),
        }
    }

    Rc::clone(&root)
}

#[test]
fn process_move_root_test() {
    let root =
        process_commands(vec![Terminal::Command(TermCommand::Cd(ChangeDir::Root))].into_iter());
    assert_eq!(
        *root,
        TreeNode::Directory(RefCell::new(DirectoryData {
            parent: None,
            name: "/".to_owned(),
            total_size: 0,
            children: vec![]
        }))
    );
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
    );

    assert_eq!(
        *root,
        TreeNode::Directory(RefCell::new(DirectoryData {
            parent: None,
            name: "/".to_owned(),
            total_size: 14,
            children: vec![TreeNode::new_file(
                Rc::downgrade(&root),
                "unk.png".to_owned(),
                14
            )]
        }))
    );
}

#[test]
fn read_file_test() {
    let root = load_file("./test.txt");
    assert_eq!(root.get_size(), 4);
}
