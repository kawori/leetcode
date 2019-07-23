// Given two arrays, write a function to compute their intersection.

// Example 1:
// Input: nums1 = [1,2,2,1], nums2 = [2,2]
// Output: [2]

// Example 2:
// Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
// Output: [9,4]

// Note:
// Each element in the result must be unique.
// The result can be in any order.

#include <iostream>
#include <set>
#include <vector>
using std::set;
using std::vector;

class Solution {
public:
  set<int> vec2set(const vector<int> &nums) {
    set<int> ret;
    for (int n : nums) {
      ret.insert(n);
    }
    return ret;
  }
  vector<int> intersection(vector<int> &nums1, vector<int> &nums2) {
    set<int> set1 = vec2set(nums1), set2 = vec2set(nums2);
    vector<int> ret;
    for (int i : set1) {
      if (set2.count(i)) {
        ret.push_back(i);
      }
    }
    return ret;
  }
};

void print_vec(const vector<int> &vec) {
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
  vector<int> nums1{1, 2, 2, 1}, nums2{2, 2};
  print_vec(nums1);
  std::cout << " ^ ";
  print_vec(nums2);
  std::cout << " -> ";
  print_vec(sol.intersection(nums1, nums2));

  return 0;
}
