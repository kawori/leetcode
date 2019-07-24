// Given an n-ary tree, return the level order traversal of its nodes' values.
// (ie, from left to right, level by level).

// Note:
// The depth of the tree is at most 1000.
// The total number of nodes is at most 5000.

import java.util.List;
import java.util.LinkedList;
import java.util.Queue;
import java.util.ArrayDeque;

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
    public List<List<Integer>> levelOrder(Node root) {
        Queue<Node> q = new ArrayDeque<Node>();
        if (root != null) {
            q.add(root);
        }
        List<List<Integer>> ret = new LinkedList<List<Integer>>();
        int numChildren = 1, cntChildren = 0, newChildren = 1;
        while (!q.isEmpty()) {
            cntChildren = 0;
            numChildren = newChildren;
            newChildren = 0;
            List<Integer> l = new LinkedList<Integer>();
            while (cntChildren < numChildren) {
                Node cur = q.remove();
                cntChildren++;
                if (cur != null) {
                    l.add(cur.val);
                    List<Node> children = cur.children;
                    if (children != null && !children.isEmpty()) {
                        q.addAll(children);
                        newChildren += children.size();
                    }
                }
            }
            ret.add(l);
        }
        return ret;
    }
}
