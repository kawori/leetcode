// Given an n-ary tree, return the postorder traversal of its nodes' values.

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
    public List<Integer> postorder1(Node root) {
        List<Integer> ret = new LinkedList<Integer>();
        if (root != null) {
            List<Node> children = root.children;
            if (children != null && !children.isEmpty()) {
                for (Node n : children) {
                    List<Integer> tmp = postorder1(n);
                    ret.addAll(tmp);
                }
            }
            ret.add(root.val);
        }
        return ret;
    }

    // iteration method
    public List<Integer> postorder(Node root) {
        Stack<Node> stack1 = new Stack<Node>(), stack2 = new Stack<Node>();
        if (root != null) {
            stack1.push(root);
        }
        while (!stack1.empty()) {
            Node cur = stack1.pop();
            stack2.push(cur);
            List<Node> children = cur.children;
            if (children != null && !children.isEmpty()) {
                for (Node n : children) {
                    stack1.push(n);
                }
            }
        }
        List<Integer> ret = new LinkedList<Integer>();
        while (!stack2.empty()) {
            ret.add(stack2.pop().val);
        }
        return ret;
    }
}