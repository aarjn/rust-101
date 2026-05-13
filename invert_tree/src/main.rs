use std::mem;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn insert(root: Option<Box<TreeNode>>, val: i32) -> Option<Box<TreeNode>> {
    match root {
        None => Some(Box::new(TreeNode::new(val))),
        Some(mut node) => {
            if val < node.val {
                node.left = insert(node.left, val);
            } else {
                node.right = insert(node.right, val);
            }
            Some(node)
        }
    }
}

pub fn invert(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    match root {
        None => None,
        Some(mut node) => {
            mem::swap(&mut node.left, &mut node.right);
            node.right = invert(node.right);
            node.left = invert(node.left);
            Some(node)
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut root = None;
    root = insert(root, 4);
    root = insert(root, 2);
    root = insert(root, 7);
    root = insert(root, 1);
    println!("root: {:?}", root);

    root = invert(root);

    println!("inverted root: {:?}", root);
}
