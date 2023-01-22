use std::{cell::RefCell, rc::Rc};

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

pub fn is_valid_bst(root: OptionSharedTreeNode) -> bool {
    is_valid_bst_ref(&root, None, None)
}

fn is_valid_bst_ref(
    node: &OptionSharedTreeNode,
    max_limit: Option<i32>,
    min_limit: Option<i32>,
) -> bool {
    match node {
        None => true,
        Some(node_rc) => {
            let val = node_rc.borrow().val;

            if let Some(right) = &node_rc.borrow().right {
                let right_val = right.borrow().val;
                if right_val <= val
                    || max_limit.map(|limit| right_val >= limit).unwrap_or(false)
                    || min_limit.map(|limit| right_val <= limit).unwrap_or(false)
                {
                    return false;
                }
            }

            if let Some(left) = &node_rc.borrow().left {
                let left_val = left.borrow().val;
                if left_val >= val
                    || max_limit.map(|limit| left_val >= limit).unwrap_or(false)
                    || min_limit.map(|limit| left_val <= limit).unwrap_or(false)
                {
                    return false;
                }
            }

            is_valid_bst_ref(&node_rc.borrow().right, max_limit, Some(val))
                && is_valid_bst_ref(&node_rc.borrow().left, Some(val), min_limit)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let right_child = Rc::new(RefCell::new(TreeNode::new(2147483647)));
        let root = Rc::new(RefCell::new(TreeNode::new(-2147483648)));
        root.borrow_mut().right = Some(right_child);
        assert!(is_valid_bst(Some(root)));
    }
}
