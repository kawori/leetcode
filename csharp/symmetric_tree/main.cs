// Given a binary tree, check whether it is a mirror of itself (ie, symmetric around its center).

// For example, this binary tree [1,2,2,3,4,4,3] is symmetric:

//     1
//    / \
//   2   2
//  / \ / \
// 3  4 4  3
//  

// But the following [1,2,2,null,3,null,3] is not:

//     1
//    / \
//   2   2
//    \   \
//    3    3
//  

// Note:
// Bonus points if you could solve it both recursively and iteratively.


using System.Collections;

// Definition for a binary tree node.
public class TreeNode {
    public int val;
    public TreeNode left;
    public TreeNode right;
    public TreeNode(int x) { val = x; }
}

public class Solution {
    public bool IsMirror(TreeNode t1, TreeNode t2) {
        if (t1 == null && t2 == null) {
            return true;
        }
        if (t1 == null || t2 == null) {
            return false;
        }
        return (t1.val == t2.val) && IsMirror(t1.left, t2.right) && IsMirror(t1.right, t2.left);
    }
    // recursive method
    public bool IsSymmetric(TreeNode root) {
        return IsMirror(root, root);
    }

    // iteration method
    public bool IsSymmetric(TreeNode root) {
        Queue q = new Queue();
        q.Enqueue(root);
        q.Enqueue(root);
        while (q.Count > 0) {
            TreeNode t1 = (TreeNode)q.Dequeue();
            TreeNode t2 = (TreeNode)q.Dequeue();
            if (t1 == null && t2 == null) {
                continue;
            }
            if (t1 == null || t2 == null) {
                return false;
            }
            if (t1.val != t2.val) {
                return false;
            }
            q.Enqueue(t1.left);
            q.Enqueue(t2.right);
            q.Enqueue(t1.right);
            q.Enqueue(t2.left);
        }
        return true;
    }
}