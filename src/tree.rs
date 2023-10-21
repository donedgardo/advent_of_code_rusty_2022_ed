pub struct TreeNode<T> {
    value: T,
    children: Vec<TreeNode<T>>
}

impl<T> TreeNode< T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            children: vec![],
        }
    }
     pub fn add_child(&mut self, node: TreeNode<T>) {
         self.children.push(node);
     }
}

pub struct Tree<T> {
    root: Option<TreeNode<T>>
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree {
            root: None
        }
    }
    pub fn create_root(&mut self, value: T) -> &mut TreeNode<T> {
        self.root = Some(TreeNode::new(value));
        self.root.as_mut().unwrap()
    }

    pub fn is_empty(&self ) -> bool {
        match self.root {
            None => { true }
            Some(_) => { false }
        }
    }
    pub fn read_depth_first(&self) -> Vec<&T> {
        let mut output: Vec<&T> = vec![];
        match &self.root {
            None => { return output }
            Some(root) => {
                Self::depth_first(&root, &mut output)
            }
        }
        output
    }

    fn depth_first<'a>(from: &'a TreeNode<T>, mut output: &mut Vec<&'a T>) {
        if from.children.is_empty() {
            output.push(&from.value);
        } else {
            for child in from.children.iter() {
                Self::depth_first(child, &mut output);
            }
            output.push(&from.value);
        }
    }
}

#[cfg(test)]
mod my_tree_test {
    use crate::tree::{Tree, TreeNode};

    #[test]
    fn it_creates_without_root() {
        let tree: Tree<u32> = Tree::new();
        assert!(tree.is_empty());
    }

    #[test]
    fn it_insert_root_on_empty_tree() {
        let mut tree = Tree::new();
        tree.create_root(1);
        assert!(!tree.is_empty());
    }

    #[test]
    fn it_can_reference_its_root() {
        let mut tree = Tree::new();
        let root = tree.create_root(1);
        assert_eq!(root.value, 1);
    }

    #[test]
    fn it_can_add_children_to_root() {
        let mut tree = Tree::new();
        let root: &mut TreeNode<i32> = tree.create_root(1);
        let child = TreeNode::new(2);
        root.add_child( child);
        assert_eq!(root.value, 1);
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].value, 2);
    }

    #[test]
    fn it_can_add_grandchild_to_root() {
        let mut tree:Tree<i32> = Tree::new();
        let root = tree.create_root(1);

        let mut child: TreeNode<i32> = TreeNode::new(2);
        let grandchild: TreeNode<i32> = TreeNode::new(3);

        child.add_child(grandchild);
        root.add_child(child);
        assert_eq!(tree.read_depth_first(), &[&3, &2, &1])
    }

    #[test]
    fn it_can_read_depth_first() {
        let mut tree:Tree<i32> = Tree::new();
        let root = tree.create_root(1);
        let mut child1: TreeNode<i32> = TreeNode::new(2);
        let child2: TreeNode<i32> = TreeNode::new(3);
        let grandchild: TreeNode<i32> = TreeNode::new(4);

        child1.add_child(grandchild);
        root.add_child(child1);
        root.add_child(child2);

        assert_eq!(tree.read_depth_first(), &[&4, &2, &3, &1])
    }

}
