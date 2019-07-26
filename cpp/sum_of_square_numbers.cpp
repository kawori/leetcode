// Given a non-negative integer c, your task is to
// decide whether there're two integers a and b such that a^2 + b^2 = c.

// Example 1:
// Input: 5
// Output: True
// Explanation: 1 * 1 + 2 * 2 = 5

// Example 2:
// Input: 3
// Output: False

#include <cmath>

class Solution {
public:
  bool judgeSquareSum(int c) {
    long long l = 0, h = std::sqrt(c);
    while (l <= h) {
      long long sum = l * l + h * h;
      if (sum == c) {
        return true;
      } else if (sum < c) {
        l++;
      } else {
        h--;
      }
    }
    return false;
  }
};