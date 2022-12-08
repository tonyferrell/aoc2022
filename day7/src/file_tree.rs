use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug, PartialEq)]
pub(crate) enum TreeNode {
    Directory(RefCell<DirectoryData>),
    File(RefCell<FileData>),
}

#[derive(Debug)]
pub(crate) struct FileData {
    pub parent: Weak<TreeNode>,
    pub name: String,
    pub size: usize,
}

impl PartialEq for FileData {
    fn eq(&self, other: &Self) -> bool {
        self.parent.ptr_eq(&other.parent)&& self.name == other.name && self.size == other.size
    }
}

#[derive(Debug)]
pub(crate) struct DirectoryData {
    pub parent: Option<Weak<TreeNode>>,
    pub name: String,
    pub total_size: usize,
    pub children: Vec<Rc<TreeNode>>,
}

impl PartialEq for DirectoryData {
    fn eq(&self, other: &Self) -> bool {
        let parent_eq = match (&self.parent, &other.parent) {
            (Some(me), Some(them)) => me.ptr_eq(them),
            (None, None) => true,
            (_, _) => false,
        };

        parent_eq && self.name == other.name && self.total_size == other.total_size && self.children == other.children
    }
}

impl TreeNode {
    pub(crate) fn new_dir(
        parent: Option<Weak<TreeNode>>,
        name: String,
    ) -> Rc<TreeNode> {
        Rc::new(TreeNode::Directory(RefCell::new(DirectoryData {
            parent,
            name,
            total_size: 0,
            children: vec![],
        })))
    }

    pub(crate) fn new_file(parent: Weak<TreeNode>, name: String, size: usize) -> Rc<TreeNode> {
        Rc::new(TreeNode::File(RefCell::new(FileData {
            parent,
            name,
            size,
        })))
    }

    pub(crate) fn get_size(&self) -> usize {
        match self {
            Self::Directory(data) => data.borrow().total_size,
            Self::File(file) => file.borrow().size,
        }
    }
}

impl DirectoryData {
    pub(crate) fn add_child(&mut self, child: Rc<TreeNode>) {
        let child_size = (*child).get_size();
        self.total_size += child_size;

        self.children.push(child);
    }
}

#[test]
fn single_file_test() {
    let root = TreeNode::new_dir(None, "/".to_owned());
    let parent = Rc::downgrade(&root);
    let child = TreeNode::new_file(parent, "foo.bar".to_owned(), 14);

    if let TreeNode::Directory(data) = &*root {
        data.borrow_mut().add_child(child);
    }
    let l = &*root;
    assert_eq!(l.get_size(), 14)
}
