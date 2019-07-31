// Given a singly linked list, determine if it is a palindrome.

// Example 1:
// Input: 1->2
// Output: false

// Example 2:
// Input: 1->2->2->1
// Output: true

// Follow up:
// Could you do it in O(n) time and O(1) space?

// Definition for singly-linked list.
struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(nullptr) {}
};

class Solution {
public:
  ListNode *find_mid(ListNode *head) {
    ListNode *f = head, *s = head;
    while (f && f->next) {
      s = s->next;
      f = f->next->next;
    }
    return s;
  }
  ListNode *reverse_list(ListNode *head) {
    ListNode *p = nullptr, *q;
    while (head) {
      q = head->next;
      head->next = p;
      p = head;
      head = q;
    }
    return p;
  }
  bool isPalindrome(ListNode *head) {
    if (!head || !head->next) {
      return true;
    }
    if (!head->next->next) {
      return head->val == head->next->val;
    }
    ListNode *mid = find_mid(head);
    ListNode *rev = reverse_list(mid);
    while (head != mid) {
      if (head->val != rev->val) {
        return false;
      }
      head = head->next;
      rev = rev->next;
    }
    return true;
  }
};