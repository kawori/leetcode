// Given an n-ary tree, return the preorder traversal of its nodes' values.

import java.util.List;
import java.util.LinkedList;
import java.util.Stack;

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
    // recursion method
    public List<Integer> preorder1(Node root) {
        List<Integer> ret = new LinkedList<Integer>();
        if (root != null) {
            ret.add(root.val);
            List<Node> children = root.children;
            if (children != null) {
                for (Node p : children) {
                    List<Integer> tmp = preorder1(p);
                    ret.addAll(tmp);
                }
            }
        }
        return ret;
    }

    // iteration method
    public List<Integer> preorder(Node root) {
        Stack<Node> stack = new Stack<Node>();
        List<Integer> ret = new LinkedList<Integer>();
        stack.push(root);
        while (!stack.empty()) {
            Node cur = stack.pop();
            if (cur != null) {
                ret.add(cur.val);
                List<Node> children = cur.children;
                if (children != null && !children.isEmpty()) {
                    for (int i = children.size() - 1; i >= 0; --i) {
                        stack.push(children.get(i));
                    }
                }
            }
        }
        return ret;
    }
}