// We define the Perfect Number is a positive integer
// that is equal to the sum of all its positive divisors except itself.
// Now, given an integer n, write a function that returns true
// when it is a perfect number and false when it is not.

// Example:
// Input: 28
// Output: True
// Explanation: 28 = 1 + 2 + 4 + 7 + 14

// Note: The input number n will not exceed 100,000,000. (1e8)

#include <cmath>
#include <iostream>
#include <numeric>
#include <vector>
using std::vector;

class Solution {
public:
  bool checkPerfectNumber(int num) {
    if (1 == num) {
      return false;
    }
    vector<int> pfs{1};
    int r = std::sqrt(num), o;
    for (int i = 2; i <= r; ++i) {
      if (num % i == 0) {
        pfs.push_back(i);
        o = num / i;
        if (o != i) {
          pfs.push_back(o);
        }
      }
    }
    return num == std::accumulate(pfs.begin(), pfs.end(), 0);
  }
};

int main() {
    Solution sol;
    std::cout << 28 << " -> ";
    if (sol.checkPerfectNumber(28)) {
        std::cout << "true\n";
    } else {
        std::cout << "false\n";
    }

    return 0;
}
