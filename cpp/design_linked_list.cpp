// Design your implementation of the linked list.
// You can choose to use the singly linked list or the doubly linked list.
// A node in a singly linked list should have two attributes: val and next.
// val is the value of the current node,
// and next is a pointer/reference to the next node.
// If you want to use the doubly linked list,
// you will need one more attribute prev to
// indicate the previous node in the linked list.
// Assume all nodes in the linked list are 0-indexed.

// Implement these functions in your linked list class:
// get(index):
// Get the value of the index-th node in the linked list.
// If the index is invalid, return -1.
// addAtHead(val):
// Add a node of value val before the first element of the linked list.
// After the insertion, the new node will be the first node of the linked list.
// addAtTail(val):
// Append a node of value val to the last element of the linked list.
// addAtIndex(index, val):
// Add a node of value val before the index-th node in the linked list.
// If index equals to the length of linked list,
// the node will be appended to the end of linked list.
// If index is greater than the length, the node will not be inserted.
// deleteAtIndex(index):
// Delete the index-th node in the linked list, if the index is valid.

// Example:
// MyLinkedList linkedList = new MyLinkedList();
// linkedList.addAtHead(1);
// linkedList.addAtTail(3);
// linkedList.addAtIndex(1, 2);  // linked list becomes 1->2->3
// linkedList.get(1);            // returns 2
// linkedList.deleteAtIndex(1);  // now the linked list is 1->3
// linkedList.get(1);            // returns 3

// Note:
// All values will be in the range of [1, 1000].
// The number of operations will be in the range of [1, 1000].
// Please do not use the built-in LinkedList library.

#include <cassert>
#include <iostream>

struct Node {
  int val;
  Node *next;
};

class MyLinkedList {
public:
  // Initialize your data structure here.
  MyLinkedList() : head_(nullptr), tail_(nullptr), length_(0) {}
  ~MyLinkedList() {
    if (head_) {
      Node *p = head_, *q = nullptr;
      while (p) {
        q = p->next;
        delete p;
        p = q;
      }
      head_ = nullptr;
      tail_ = nullptr;
    }
  }

  void print() {
    if (head_) {
      Node *p = head_;
      while (p) {
        std::cout << p->val << " -> ";
        p = p->next;
      }
      std::cout << "nullptr\n";
    }
  }

  // Get the value of the index-th node in the linked list.
  // If the index is invalid, return -1.
  int get(int index) {
    if (length_ > index && 0 <= index) {
      Node *p = getPtr(index);
      return p->val;
    } else {
      return -1;
    }
  }

  // Add a node of value val before the first element of the linked list.
  // After insertion, the new node will be the first node of the linked list.
  void addAtHead(int val) {
    Node *p = new Node;
    if (p) {
      ++length_;
      p->val = val;
      p->next = head_;
      head_ = p;
      if (!tail_) // list was empty
        tail_ = p;
    } else {
      std::cerr << "failed to new Node" << std::endl;
    }
  }

  // Append a node of value val to the last element of the linked list.
  void addAtTail(int val) {
    Node *p = new Node;
    if (p) {
      ++length_;
      p->val = val;
      p->next = nullptr;
      if (tail_) { // list was not empty
        tail_->next = p;
      } else { // list was empty
        head_ = p;
      }
      tail_ = p;
    } else {
      std::cerr << "failed to new Node" << std::endl;
    }
  }

  /** Add a node of value val before the index-th node in the linked list. If
   * index equals to the length of linked list, the node will be appended to the
   * end of linked list. If index is greater than the length, the node will not
   * be inserted. */
  void addAtIndex(int index, int val) {
    if (length_ == index) { // if list was empty, index must be 0
      addAtTail(val);
    } else if (length_ > index && 0 <= index) {
      if (0 == index) {
        addAtHead(val);
      } else {
        Node *p = getPtr(index - 1), *q = p->next;
        Node *t = new Node;
        if (t) {
          ++length_;
          t->val = val;
          t->next = q;
          p->next = t;
        } else {
          std::cerr << "failed to new Node" << std::endl;
        }
      }
    }
  }

  // Delete the index-th node in the linked list, if the index is valid.
  void deleteAtIndex(int index) {
    if (head_ && length_ > index && 0 <= index) {
      if (1 == length_) {
        if (0 == index) {
          delete head_;
          head_ = nullptr;
          tail_ = nullptr;
          --length_;
        }
      } else {
        if (0 == index) {
          Node *p = head_;
          head_ = head_->next;
          delete p;
          p = nullptr;
          --length_;
        } else if (length_ - 1 == index) {
          delete tail_;
          --length_;
          tail_ = getPtr(index - 1);
          tail_->next = nullptr;
        } else {
          Node *p = getPtr(index - 1), *q = p->next;
          p->next = q->next;
          delete q;
          q = nullptr;
          --length_;
        }
      }
    }
  }

private:
  Node *getPtr(int index) {
    assert(length_ > index && 0 <= index);
    Node *p = head_;
    for (int i = 0; i < index; ++i) {
      p = p->next;
    }
    return p;
  }

private:
  Node *head_, *tail_;
  int length_;
};

int main() {
  MyLinkedList linkedList;
  linkedList.addAtHead(1);
  linkedList.addAtTail(3);
  linkedList.print();
  linkedList.addAtIndex(1, 2); // linked list becomes 1->2->3
  linkedList.print();
  assert(linkedList.get(1) == 2); // returns 2
  linkedList.deleteAtIndex(1);    // now the linked list is 1->3
  linkedList.print();
  assert(linkedList.get(1) == 3); // returns 3

  return 0;
}
