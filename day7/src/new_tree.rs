use std::{cell::RefCell, collections::HashSet, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct DirectoryNode {
    pub parent: Option<Rc<RefCell<DirectoryNode>>>,
    pub name: String,
    pub size: usize,
    pub directory_children: Vec<Rc<RefCell<DirectoryNode>>>,
    pub file_children: HashSet<String>,
}

impl DirectoryNode {
    pub fn new(parent: Option<Rc<RefCell<DirectoryNode>>>, name: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            parent,
            name,
            size: 0,
            directory_children: vec![],
            file_children: HashSet::new(),
        }))
    }

    pub fn get_child(&self, name: &str) -> Option<Rc<RefCell<Self>>> {
            self.directory_children
                .iter()
                .find(|&x| (*x).borrow().name == name)
                .map(|x| Rc::clone(x))
    }

    pub fn add_child_dir(&mut self, new: Rc<RefCell<DirectoryNode>>) {
        let new_size = new.borrow().size;
        if new_size > 0 {
            self.size += new_size;
        }

        self.directory_children.push(new);
    }

    pub fn add_child_file(&mut self, name: String, size: usize) {
        if self.file_children.insert(name) {
            self.size += size
        }
    }
}
