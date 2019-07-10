// Implement int sqrt(int x).

// Compute and return the square root of x, where x is guaranteed to be a
// non-negative integer.

// Since the return type is an integer, the decimal digits are truncated and
// only the integer part of the result is returned.

// Example 1:
// Input: 4
// Output: 2

// Example 2:
// Input: 8
// Output: 2
// Explanation: The square root of 8 is 2.82842..., and since the decimal part
// is truncated, 2 is returned.

#include <cstdio>

class Solution {
public:
  // binary method
  int mySqrt(int x) {
    if (0 == x || 1 == x) {
      return x;
    }
    int l = 1, h = x / 2 + 1;
    while (l <= h) {
      long long m = l + (h - l) / 2;
      long long s = m * m;
      if (s == x) {
        return m;
      } else if (s < x) {
        l = m + 1;
      } else {
        h = m - 1;
      }
    }
    return h;
  }

  // Newton method
  int mySqrt2(int x) {
    if (0 == x || 1 == x) {
      return x;
    }
    double pre = 0.0, cur = 1.0; // double precision is necessary
    while (pre != cur) {
        pre = cur;
        cur = (cur + x / cur) / 2;
    }
    return cur;
  }
};

int main() {
    Solution sol;
    std::printf("binary: %d -> %d\n", 4, sol.mySqrt(4));
    std::printf("binary: %d -> %d\n", 8, sol.mySqrt(8));
    std::printf("Newton: %d -> %d\n", 4, sol.mySqrt2(4));
    std::printf("Newton: %d -> %d\n", 8, sol.mySqrt2(8));

}