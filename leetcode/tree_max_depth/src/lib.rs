use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

type OptionSharedTreeNode = Option<Rc<RefCell<TreeNode>>>;

pub fn max_depth(root: OptionSharedTreeNode) -> i32 {
    max_depth_recursive(&root, 0)
}

fn max_depth_recursive(maybe_node: &OptionSharedTreeNode, depth: i32) -> i32 {
    match maybe_node {
        Some(node) => max_depth_recursive(&node.borrow().right, depth + 1)
            .max(max_depth_recursive(&node.borrow().left, depth + 1)),
        None => depth,
    }
}

#[cfg(test)]
mod tests {}
