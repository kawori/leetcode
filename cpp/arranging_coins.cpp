// You have a total of n coins that you want to form in a staircase shape,
// where every k-th row must have exactly k coins.

// Given n, find the total number of full staircase rows that can be formed.

// n is a non-negative integer
// and fits within the range of a 32-bit signed integer.

// Example 1:
// n = 5
// The coins can form the following rows:
// ¤
// ¤ ¤
// ¤ ¤
// Because the 3rd row is incomplete, we return 2.

// Example 2:
// n = 8
// The coins can form the following rows:
// ¤
// ¤ ¤
// ¤ ¤ ¤
// ¤ ¤
// Because the 4th row is incomplete, we return 3.

#include <cmath>
#include <cstdio>

class Solution {
public:
  int arrangeCoins(int n) {
    // (1+m)*m/2<=n
    // m^2+m-2*n<=0
    // m <= (-1+(1+8*n)^0.5)/2
    return std::floor(-0.5 + std::sqrt(0.25 + 2.0 * n));
  }
};

int main() {
  Solution sol;
  std::printf("%d -> %d\n", 8, sol.arrangeCoins(8));
  return 0;
}
