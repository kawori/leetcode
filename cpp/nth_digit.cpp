// Find the nth digit of the infinite integer sequence
// 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...
// Note:
// n is positive and
// will fit within the range of a 32-bit signed integer (n < 231).

// Example 1:
// Input: 3
// Output: 3

// Example 2:
// Input: 11
// Output: 0

// Explanation:
// The 11th digit of the sequence 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...
// is a 0, which is part of the number 10.

#include <iostream>
#include <vector>
using std::vector;

class Solution {
private:
  vector<int> gaps{9, 189, 2889, 38889, 488889, 5888889, 68888889, 788888889};

public:
  int findNthDigit(int n) {
    if (10 > n) {
      return n;
    }
    int len = gaps.size();
    int i = len - 1;
    while (i >= 0 && gaps[i] > n) {
      --i;
    }
    len = i + 2;
    int w = n - gaps[len - 2];
    int tar = pow(10, len - 1) + (w - 1) / len;
    if (0 != w % len) {
      tar /= pow(10, len - w % len);
    }
    return tar % 10;
  }
};

int main() {
  Solution sol;
  std::cout << 11 << " -> " << sol.findNthDigit(11) << std::endl;

  return 0;
}
