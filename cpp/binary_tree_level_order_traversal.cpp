// Given a binary tree, return the level order traversal of its nodes' values.
// (ie, from left to right, level by level).

// For example:
// Given binary tree [3,9,20,null,null,15,7],
//     3
//    / \
//   9  20
//     /  \
//    15   7
// return its level order traversal as:
// [
//   [3],
//   [9,20],
//   [15,7]
// ]

#include <vector>
#include <deque>
using std::vector;
using std::deque;

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
public:
  vector<vector<int>> levelOrder(TreeNode *root) {
      vector<vector<int>> ret;
      deque<TreeNode *> q1, q2;
      if (root != nullptr) {
          q1.push_back(root);
      }
      while (!q1.empty() || !q2.empty()) {
          vector<int> level;
          if (!q1.empty()) {
              for (auto elem : q1) {
                  level.push_back(elem->val);
              }
              while (!q1.empty()) {
                  TreeNode *p = q1.front();
                  if (p->left != nullptr) {
                      q2.push_back(p->left);
                  }
                  if (p->right != nullptr) {
                      q2.push_back(p->right);
                  }
                  q1.pop_front();
              }
          } else {
              for (auto elem : q2) {
                  level.push_back(elem->val);
              }
              while (!q2.empty()) {
                  TreeNode *p = q2.front();
                  if (p->left != nullptr) {
                      q1.push_back(p->left);
                  }
                  if (p->right != nullptr) {
                      q1.push_back(p->right);
                  }
                  q2.pop_front();
              }
          }
          ret.push_back(level);
      }
      return ret;
  }
};