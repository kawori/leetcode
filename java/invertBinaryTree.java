// Invert a binary tree.

// Example:

// Input:

//      4
//    /   \
//   2     7
//  / \   / \
// 1   3 6   9

// Output:

//      4
//    /   \
//   7     2
//  / \   / \
// 9   6 3   1


// Definition for a binary tree node.
public class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;
    TreeNode(int x) { val = x; }
}

class Solution {
    public TreeNode invertTree(TreeNode root) {
        TreeNode ret = null;
        if (root != null) {
            ret = new TreeNode(root.val);
            ret.right = invertTree(root.left);
            ret.left = invertTree(root.right);
        }
        return ret;
    }
}