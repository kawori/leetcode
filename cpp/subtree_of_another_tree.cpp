// Given two non-empty binary trees s and t,check whether
// tree t has exactly the same structure and node values with a subtree of s.
// A subtree of s is a tree consists of a node in s and all of this node's
// descendants. The tree s could also be considered as a subtree of itself.

// Example 1:
// Given tree s:
//      3
//     / \
//    4   5
//   / \
//  1   2
// Given tree t:
//    4
//   / \
//  1   2
// Return true, because t has the same structure and node values with a subtree
// of s.

// Example 2:
// Given tree s:
//      3
//     / \
//    4   5
//   / \
//  1   2
//     /
//    0
// Given tree t:
//    4
//   / \
//  1   2
// Return false.

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
public:
  bool isSametree(TreeNode *s, TreeNode *t) {
    if (!s && !t) {
      return true;
    } else if (!s && t || s && !t) {
      return false;
    }
    return s->val == t->val && isSametree(s->left, t->left) &&
           isSametree(s->right, t->right);
  }
  bool isSubtree(TreeNode *s, TreeNode *t) {
    if (isSametree(s, t)) {
      return true;
    }
    if (!s->left && !s->right) {
      return false;
    }
    if (!s->left) {
      return isSubtree(s->right, t);
    }
    if (!s->right) {
      return isSubtree(s->left, t);
    }
    return isSubtree(s->left, t) || isSubtree(s->right, t);
  }
};
