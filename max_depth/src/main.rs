#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn insert(root: Option<Box<TreeNode>>, val: i32) -> Option<Box<TreeNode>> {
        match root {
            None => Some(Box::new(TreeNode::new(val))),
            Some(mut node) => {
                if val < node.val {
                    node.left = TreeNode::insert(node.left, val);
                } else {
                    node.right = TreeNode::insert(node.right, val);
                }
                Some(node)
            }
        }
    }

    pub fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                i32::max(
                    TreeNode::max_depth(node.left),
                    TreeNode::max_depth(node.right),
                ) + 1
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut root = None;
    root = TreeNode::insert(root, 4);
    root = TreeNode::insert(root, 5);
    root = TreeNode::insert(root, 3);
    root = TreeNode::insert(root, 7);
    root = TreeNode::insert(root, 2);

    println!("Tree: {:?}", root);

    let depth = TreeNode::max_depth(root);
    println!("Depth:{}", depth);
}
