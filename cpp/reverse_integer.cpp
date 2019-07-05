// Given a 32-bit signed integer, reverse digits of an integer.

// Example 1:
// Input: 123
// Output: 321

// Example 2:
// Input: -123
// Output: -321

// Example 3:
// Input: 120
// Output: 21

// Note:
// Assume we are dealing with an environment which could only store integers
// within the 32-bit signed integer range: [−2^31, 2^31 − 1]. For the purpose of
// this problem, assume that your function returns 0 when the reversed integer
// overflows.

#include <cmath>
#include <cstdio>
#include <limits.h>
#include <vector>
using std::vector;

class Solution {
public:
  int reverse(int x) {
    // long long will be at least 64 bits
    // necessary for int overflow
    long long y = x;
    if (x < 0) {
      y = -y;
    }
    vector<int> digits;
    while (y) {
      digits.push_back(y % 10);
      y /= 10;
    }
    int digit_num = digits.size();
    long long ret = 0;
    for (int i = 0; i < digit_num; ++i) {
      ret += std::pow(10, digit_num - 1 - i) * digits[i];
    }
    if (x < 0) {
      ret = -ret;
    }
    if (ret < INT_MIN || ret > INT_MAX) {
      return 0;
    }
    return ret;
  }
};

int main() {
  int x = INT_MIN;
  Solution sol;
  int y = sol.reverse(x);
  std::printf("%d %d\n", x, y);
  x = -123;
  y = sol.reverse(x);
  std::printf("%d %d\n", x, y);
  x = 567;
  y = sol.reverse(x);
  std::printf("%d %d\n", x, y);
  x = INT_MAX;
  y = sol.reverse(x);
  std::printf("%d %d\n", x, y);
  return 0;
}
