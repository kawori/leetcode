// Given an array where elements are sorted in ascending order, convert it to a height balanced BST.
// For this problem, a height-balanced binary tree is defined as
// a binary tree in which the depth of the two subtrees of every node never differ by more than 1.

// Example:
// Given the sorted array: [-10,-3,0,5,9],
// One possible answer is: [0,-3,9,-10,null,5], which represents the following height balanced BST:
//       0
//      / \
//    -3   9
//    /   /
//  -10  5

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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        Solution::build_bst(&nums, 0, (nums.len() - 1) as i32)
    }
    pub fn build_bst(nums: &Vec<i32>, l: i32, r: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if l > r {
            return None;
        }
        let m = (l + r) / 2;
        let left = Solution::build_bst(nums, l, m - 1);
        let right = Solution::build_bst(nums, m + 1, r);
        let root = Rc::new(RefCell::new(TreeNode::new(nums[m as usize])));
        root.borrow_mut().left = left;
        root.borrow_mut().right = right;
        Some(root)
    }
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let mut q1: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut q2: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        if let Some(node) = root {
            q1.push_back(node);
        }
        while !q1.is_empty() || !q2.is_empty() {
            let mut level: Vec<i32> = Vec::new();
            if !q1.is_empty() {
                for node in &q1 {
                    level.push(node.borrow().val);
                }
                while !q1.is_empty() {
                    let p = q1.front().unwrap();
                    if let Some(node) = &p.borrow().left {
                        q2.push_back(Rc::clone(node));
                    }
                    if let Some(node) = &p.borrow().right {
                        q2.push_back(Rc::clone(node));
                    }
                    q1.pop_front();
                }
            } else {
                for node in &q2 {
                    level.push(node.borrow().val);
                }
                while !q2.is_empty() {
                    let p = q2.front().unwrap();
                    if let Some(node) = &p.borrow().left {
                        q1.push_back(Rc::clone(node));
                    }
                    if let Some(node) = &p.borrow().right {
                        q1.push_back(Rc::clone(node));
                    }
                    q2.pop_front();
                }
            }
            ret.push(level);
        }
        ret
    }
}

fn main() {
    let bst = Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]);
    println!("{:?}", Solution::level_order(bst));
}
