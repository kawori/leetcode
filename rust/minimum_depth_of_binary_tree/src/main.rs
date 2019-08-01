// Given a binary tree, find its minimum depth.
// The minimum depth is the number of nodes along the shortest path
// from the root node down to the nearest leaf node.
// Note: A leaf is a node with no children.

// Example:
// Given binary tree [3,9,20,null,null,15,7],
//     3
//    / \
//   9  20
//     /  \
//    15   7
// return its minimum depth = 2.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
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

struct Solution;

impl Solution {
    // recursion method
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                return 1;
            }
            if node.borrow().left.is_none() {
                return Solution::min_depth(node.borrow().right.clone()) + 1;
            }
            if node.borrow().right.is_none() {
                return Solution::min_depth(node.borrow().left.clone()) + 1;
            }
            return std::cmp::min(
                Solution::min_depth(node.borrow().right.clone()),
                Solution::min_depth(node.borrow().left.clone()),
            ) + 1;
        }
        0
    }

    // iteration method, level order scan
    pub fn min_depth2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let mut list: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
            list.push_back((node, 1));
            while !list.is_empty() {
                if let Some(cur) = list.pop_front() {
                    if cur.0.borrow().left.is_none() && cur.0.borrow().right.is_none() {
                        return cur.1;
                    }
                    if let Some(left) = cur.0.borrow().left.clone() {
                        list.push_back((left, cur.1 + 1));
                    }
                    if let Some(right) = cur.0.borrow().right.clone() {
                        list.push_back((right, cur.1 + 1));
                    }
                }
            }
        }
        0
    }
}

fn main() {
    println!("Hello, world!");
}
