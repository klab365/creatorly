use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct FileTree {
    name: String,
    children: Vec<Rc<RefCell<FileTree>>>,
    parent: Option<Rc<RefCell<FileTree>>>,
}

impl FileTree {
    pub fn new(name: String) -> Self {
        Self {
            name,
            children: vec![],
            parent: None,
        }
    }

    pub fn add_child(&mut self, child: Rc<RefCell<FileTree>>) {
        self.children.push(child);
    }

    pub fn set_parent(&mut self, parent: Rc<RefCell<FileTree>>) {
        self.parent = Some(parent);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_the_expected_folder() {
        let folder = FileTree::new("folder".to_string());

        assert_eq!(folder.name, "folder");
        assert_eq!(folder.children.len(), 0);
        assert_eq!(folder.parent, None);
    }

    #[test]
    fn should_add_child_to_folder() {
        let mut folder = FileTree::new("folder".to_string());
        let child = Rc::new(RefCell::new(FileTree::new("child".to_string())));

        folder.add_child(child.clone());

        assert_eq!(folder.children.len(), 1);
        assert_eq!(folder.children[0], child);
    }

    #[test]
    fn should_set_parent_to_folder() {
        let mut folder = FileTree::new("folder".to_string());
        let parent = Rc::new(RefCell::new(FileTree::new("parent".to_string())));

        folder.set_parent(parent.clone());

        assert_eq!(folder.parent, Some(parent));
    }
}
