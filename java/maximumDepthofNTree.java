// Given a n-ary tree, find its maximum depth.
// The maximum depth is the number of nodes along the longest path
// from the root node down to the farthest leaf node.

// Note:
// The depth of the tree is at most 1000.
// The total number of nodes is at most 5000.

import java.util.List;

// Definition for a Node.
class Node {
    public int val;
    public List<Node> children;

    public Node() {
    }

    public Node(int _val, List<Node> _children) {
        val = _val;
        children = _children;
    }
};

class Solution {
    public int maxDepth(Node root) {
        if (root != null) {
            List<Node> children = root.children;
            int child_max_depth = 0;
            if (children != null && !children.isEmpty()) {
                for (Node child : children) {
                    int tmp_depth = maxDepth(child);
                    if (tmp_depth > child_max_depth) {
                        child_max_depth = tmp_depth;
                    }
                }
            }
            return 1 + child_max_depth;
        } else {
            return 0;
        }
    }
}
