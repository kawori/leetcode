// Reverse a singly linked list.

// Example:

// Input: 1->2->3->4->5->NULL
// Output: 5->4->3->2->1->NULL

import java.util.Stack;

// Definition for singly-linked list.
class ListNode {
    int val;
    ListNode next;

    ListNode(int x) {
        val = x;
    }
}

class Solution {
    public ListNode reverseList1(ListNode head) {
        Stack<Integer> my_stack = new Stack<Integer>();
        while (head != null) {
            my_stack.push(head.val);
            head = head.next;
        }
        if (my_stack.empty()) {
            return null;
        }
        ListNode ret = new ListNode(my_stack.pop());
        ListNode p = ret;
        while (!my_stack.empty()) {
            p.next = new ListNode(my_stack.pop());
            p = p.next;
        }
        return ret;
    }

    public ListNode reverseList(ListNode head) {
        if (head == null || head.next == null) {
            return head;
        }
        ListNode p = head, q = head.next;
        head.next = null;
        while (q != null) {
            ListNode tmp = q.next;
            q.next = p;
            p = q;
            q = tmp;
        }
        return p;
    }
}
