//8. Given a binary tree, implement a function that returns the maximum depth of the tree
//importing smart pointer types
use std::rc::Rc;//track of no of ref
use std::cell::RefCell;//mutability 

//defining a struct to represent nodes in a binary tree.
#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

//implementation block for the TreeNode struct.
impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

type BinaryTree = Option<Rc<RefCell<TreeNode>>>;

//max_depth function that uses recursion to find the depth
fn max_depth(root: &BinaryTree) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(&node.borrow().left);
            let right_depth = max_depth(&node.borrow().right);
            //including the root node, so 1 +
            1 + std::cmp::max(left_depth, right_depth)
        },
        // If the node does not exist
        None => 0,
    }
}

fn main() {
    //creating nodes
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let left1 = Rc::new(RefCell::new(TreeNode::new(2)));
    let right1 = Rc::new(RefCell::new(TreeNode::new(3)));
    let right2 = Rc::new(RefCell::new(TreeNode::new(4)));
    let left2 = Rc::new(RefCell::new(TreeNode::new(5))); 

    //constructing the tree
    root.borrow_mut().left = Some(left1.clone());
    root.borrow_mut().right = Some(right1.clone());
    right1.borrow_mut().right = Some(right2.clone());
    right2.borrow_mut().left = Some(left2.clone());

    println!("Maximum depth of the tree: {}", max_depth(&Some(root)));
}
