// Definition for singly-linked list.
class ListNode {
    constructor(val) {
        this.val = val;
        this.next = null;
    }
}

/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var deleteDuplicates = function(head) {
    var p = head;
    if (p != null) {
        var q = p.next;
    }
    while (p != null) {
        while (q != null && q.val == p.val) {
            q = q.next;
        }
        p.next = q;
        p = q;
    }
    return head;
};