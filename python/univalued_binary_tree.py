# A binary tree is univalued if every node in the tree has the same value.
# Return true if and only if the given tree is univalued.

# Note:
# The number of nodes in the given tree will be in the range [1, 100].
# Each node's value will be an integer in the range [0, 99].


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def isUnivalTree(self, root: TreeNode) -> bool:
        if root:
            if root.left:
                if not self.isUnivalTree(
                        root.left) or root.val != root.left.val:
                    return False
            if root.right:
                if not self.isUnivalTree(
                        root.right) or root.val != root.right.val:
                    return False
        return True
