use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct TreeNode { 
    pub value: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, node: TreeNode) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: TreeNode) -> Self {
        self.right = Some(Box::new(node));
        self
    }


    pub fn insert(&mut self, insert_value: i32) {
        let mut queue: VecDeque<&mut TreeNode> = VecDeque::new();
        queue.push_front(self);

        loop {
            let TreeNode {
                ref mut left,
                ref mut right,
                ref value,
                ..
            } = queue.pop_back().unwrap();


            if insert_value < *value {
                match left {
                    Some(node) => {
                        queue.push_front(node)
                    },
                    None => {
                        *left = Some(Box::new(TreeNode::new(insert_value)));
                        return
                    }
                }
            } else {
                match right {
                    Some(node) => {
                        queue.push_front(node)
                    },
                    None => {
                        *right = Some(Box::new(TreeNode::new(insert_value)));
                        return;
                    }
                }
            }
        }
    }

    pub fn find(&mut self, search_value: i32) -> bool {
        let mut queue: VecDeque<&mut TreeNode> = VecDeque::new();
        queue.push_front(self);

        loop {
            let TreeNode {
                ref mut left,
                ref mut right,
                ref value,
                ..
            } = queue.pop_back().unwrap();

            if *value == search_value {
                return true;
            }

            match left {
                Some(node) => { queue.push_front(node) },
                None => {}
            }

            match right {
                Some(node) => { queue.push_front(node) },
                None => {}
            }

            if queue.is_empty() {
                return false;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_tree() {
        let tree = TreeNode::new(1);

        assert_eq!(tree.value, 1);
    }

    #[test]
    fn insert_left() {
        let tree = TreeNode::new(1).left(TreeNode::new(2));

        if let Some(node) = tree.left {
            assert_eq!(node.value, 2);
        }

        assert_eq!(tree.right, None);
    }

    #[test]
    fn insert_right() {
        let tree = TreeNode::new(1).right(TreeNode::new(2));

        if let Some(node) = tree.right {
            assert_eq!(node.value, 2);
        }

        assert_eq!(tree.left, None);
    }

    #[test]
    fn test_insert() {
        let mut tree: TreeNode = TreeNode::new(2);
        tree.insert(1);
        tree.insert(4);
        tree.insert(3);
        tree.insert(5);

        assert_eq!(
            tree,
            TreeNode::new(2)
                .left(
                    TreeNode::new(1)
                )
                .right(
                    TreeNode::new(4)
                        .left(TreeNode::new(3))
                        .right(TreeNode::new(5))
                )
        );

        tree.insert(6);

        assert_eq!(
            tree,
            TreeNode::new(2)
                .left(
                    TreeNode::new(1)
                )
                .right(
                    TreeNode::new(4)
                        .left(TreeNode::new(3))
                        .right(
                            TreeNode::new(5)
                            .right(TreeNode::new(6))
                        )
                )
        );
    }

    #[test]
    fn test_find() {
        let mut tree: TreeNode = TreeNode::new(1);
        tree.insert(2);
        tree.insert(3);
        tree.insert(4);
        tree.insert(5);

        assert_eq!(tree.find(1), true);
        assert_eq!(tree.find(2), true);
        assert_eq!(tree.find(3), true);
        assert_eq!(tree.find(4), true);
        assert_eq!(tree.find(7), false);
    }
}
