// The set S originally contains numbers from 1 to n.
// But unfortunately, due to the data error,
// one of the numbers in the set got duplicated to another number in the set,
// which results in repetition of one number and loss of another number.

// Given an array nums representing the data status of this set after the error.
// Your task is to firstly find the number occurs twice
// and then find the number that is missing.
// Return them in the form of an array.

// Example 1:
// Input: nums = [1,2,2,4]
// Output: [2,3]

// Note:
// The given array size will in the range [2, 10000].
// The given array's numbers won't have any order.

#include <iostream>
#include <vector>
using std::vector;

class Solution {
public:
  vector<int> findErrorNums(vector<int> &nums) {
    int n = nums.size();
    vector<int> ret(2, 0);
    vector<int> my_set(n + 1, 0);
    int sum = 0;
    for (int i = 0; i < n; ++i) {
      if (my_set[nums[i]]) {
        ret[0] = nums[i];
      } else {
        sum += nums[i];
        my_set[nums[i]] = 1;
      }
    }
    ret[1] = (1 + n) * n / 2 - sum;
    return ret;
  }
};

int main() {
  Solution sol;
  vector<int> nums{1, 2, 2, 4};
  vector<int> ret = sol.findErrorNums(nums);
  std::cout << "[1, 2, 2, 4] -> [" << ret[0] << ", " << ret[1] << "]\n";

  return 0;
}