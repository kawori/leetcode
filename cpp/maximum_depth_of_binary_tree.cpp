// Given a binary tree, find its maximum depth.

// The maximum depth is the number of nodes along the longest path from the root
// node down to the farthest leaf node.

// Note: A leaf is a node with no children.

// Example:

// Given binary tree [3,9,20,null,null,15,7],

//     3
//    / \
//   9  20
//     /  \
//    15   7
// return its depth = 3.

// Definition for a binary tree node.
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
public:
    int maxDepth(TreeNode* root) {
        int ret = 0;
        if (root) {
            ret++;
            int left_depth = maxDepth(root->left);
            int right_depth = maxDepth(root->right);
            ret += left_depth > right_depth ? left_depth : right_depth;
        }
        return ret;
    }
};
