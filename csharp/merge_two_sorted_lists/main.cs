// Definition for singly-linked list.
public class ListNode {
    public int val;
    public ListNode next;
    public ListNode(int x) { val = x; }
}

public class Solution {
    public ListNode MergeTwoLists(ListNode l1, ListNode l2) {
        ListNode ret = new ListNode(0);
        ret.next = null;
        if (l1 != null && l2 != null) {
            if (l1.val < l2.val) {
                ret.val = l1.val;
                l1 = l1.next;
            } else {
                ret.val = l2.val;
                l2 = l2.next;
            }
        } else if (l1 != null) {
            ret.val = l1.val;
            l1 = l1.next;
        } else if (l2 != null) {
            ret.val = l2.val;
            l2 = l2.next;
        } else {
            return null;
        }
        ListNode p = ret;
        while (l1 != null && l2 != null) {
            if (l1.val < l2.val) {
                ListNode q = new ListNode(l1.val);
                q.next = null;
                p.next = q;
                p = q;
                l1 = l1.next;
            } else {
                ListNode q = new ListNode(l2.val);
                q.next = null;
                p.next = q;
                p = q;
                l2 = l2.next;
            }
        }
        if (l1 == null && l2 != null) {
            p.next = l2;
        } else if (l1 != null && l2 == null) {
            p.next = l1;
        }
        return ret;
    }
}