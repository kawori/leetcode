// Design a class to find the kth largest element in a stream.
// Note that it is the kth largest element in the sorted order,
// not the kth distinct element.

// Your KthLargest class will have a constructor
// which accepts an integer k and an integer array nums,
// which contains initial elements from the stream.
// For each call to the method KthLargest.add,
// return the element representing the kth largest element in the stream.

// Example:
// int k = 3;
// int[] arr = [4,5,8,2];
// KthLargest kthLargest = new KthLargest(3, arr);
// kthLargest.add(3);   // returns 4
// kthLargest.add(5);   // returns 5
// kthLargest.add(10);  // returns 5
// kthLargest.add(9);   // returns 8
// kthLargest.add(4);   // returns 8

// Note:
// You may assume that nums' length ≥ k-1 and k ≥ 1.

#include <cstdio>
#include <functional>
#include <queue>
#include <vector>
using std::vector;
#include "utils.h"

class KthLargest {
private:
  std::priority_queue<int, vector<int>, std::greater<int>> min_heap_;
  int k_;

public:
  KthLargest(int k, vector<int> nums) : k_(k) {
    int len = nums.size();
    if (k <= len) {
      for (int i = 0; i < k; ++i) {
        min_heap_.push(nums[i]);
      }
      for (int i = k; i < len; ++i) {
        if (nums[i] > min_heap_.top()) {
          min_heap_.pop();
          min_heap_.push(nums[i]);
        }
      }
    } else {
      for (int i = 0; i < len; ++i) {
        min_heap_.push(nums[i]);
      }
    }
  }

  int add(int val) {
    int len = min_heap_.size();
    if (len < k_) {
      min_heap_.push(val);
    } else if (val > min_heap_.top()) {
      min_heap_.pop();
      min_heap_.push(val);
    }
    return min_heap_.top();
  }
};

// Your KthLargest object will be instantiated and called as such:
// KthLargest obj = new KthLargest(k, nums);
// int param_1 = obj.add(val);

int main() {
  vector<int> nums{4, 5, 8, 2};
  KthLargest obj(3, nums);
  std::cout << "nums: " << nums << ", k: 3\n";
  std::printf("add(3) -> %d\n", obj.add(3));
  std::printf("add(5) -> %d\n", obj.add(5));
  std::printf("add(10) -> %d\n", obj.add(10));
  std::printf("add(9) -> %d\n", obj.add(9));
  std::printf("add(4) -> %d\n", obj.add(4));

  return 0;
}
