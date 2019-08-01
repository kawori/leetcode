// Given an array, rotate the array to the right by k steps,
// where k is non-negative.

// Example 1:
// Input: [1,2,3,4,5,6,7] and k = 3
// Output: [5,6,7,1,2,3,4]
// Explanation:
// rotate 1 steps to the right: [7,1,2,3,4,5,6]
// rotate 2 steps to the right: [6,7,1,2,3,4,5]
// rotate 3 steps to the right: [5,6,7,1,2,3,4]

// Example 2:
// Input: [-1,-100,3,99] and k = 2
// Output: [3,99,-1,-100]
// Explanation:
// rotate 1 steps to the right: [99,-1,-100,3]
// rotate 2 steps to the right: [3,99,-1,-100]

// Note:
// Try to come up as many solutions as you can,
// there are at least 3 different ways to solve this problem.
// Could you do it in-place with O(1) extra space?

#include <iostream>
#include <vector>
using std::vector;

class Solution {
public:
  void rotate1(vector<int> &nums, int k) {
    int len = nums.size();
    k %= len;
    if (!k) {
      return;
    }
    vector<int> tmp(nums);
    for (int i = 0; i < len; ++i) {
      tmp[(i + k) % len] = nums[i];
    }
    nums = tmp;
  }

  void rotate2(vector<int> &nums, int k) {
    int len = nums.size();
    k %= len;
    if (!k) {
      return;
    }
    int count = 0;
    for (int i = 0; count < len; ++i) {
      int prev = nums[i], cur = i, next = i;
      do {
        next = (cur + k) % len;
        int tmp = nums[next];
        nums[next] = prev;
        prev = tmp;
        count++;
        cur = next;
      } while (next != i);
    }
  }

  void reverse(vector<int> &nums, int l, int r) {
    int len = r - l + 1;
    for (int i = 0; i < len / 2; ++i) {
      std::swap(nums[l + i], nums[r - i]);
    }
  }
  void rotate3(vector<int> &nums, int k) {
    int len = nums.size();
    k %= len;
    if (!k) {
      return;
    }
    reverse(nums, 0, len - 1);
    reverse(nums, 0, k - 1);
    reverse(nums, k, len - 1);
  }
};

template <typename T> void print_vec(const vector<T> &vec) {
  if (vec.empty()) {
    std::cout << "[]";
    return;
  }
  int len = vec.size();
  if (1 == len) {
    std::cout << "[ " << vec[0] << " ]";
    return;
  }
  std::cout << "[ ";
  for (int i = 0; i < len - 1; ++i) {
    std::cout << vec[i] << ", ";
  }
  std::cout << vec[len - 1] << " ]";
}

int main() {
  Solution sol;
  vector<int> nums{1, 2, 3, 4, 5, 6, 7};
  print_vec(nums);
  std::cout << ", 3 -> ";
  sol.rotate3(nums, 3);
  print_vec(nums);
  std::cout << std::endl;

  return 0;
}
