// Given an array of integers A sorted in non-decreasing order,
// return an array of the squares of each number, also in sorted non-decreasing
// order.

// Example 1:
// Input: [-4,-1,0,3,10]
// Output: [0,1,9,16,100]

// Example 2:
// Input: [-7,-3,2,3,11]
// Output: [4,9,9,49,121]

// Note:
// 1 <= A.length <= 10000
// -10000 <= A[i] <= 10000
// A is sorted in non-decreasing order.

#include <algorithm>
#include <cmath>
#include <iostream>
#include <vector>
using std::vector;

class Solution {
public:
  static bool cmp(int a, int b) { return std::abs(a) < std::abs(b); }
  vector<int> sortedSquares1(vector<int> &A) {
    std::sort(A.begin(), A.end(), cmp);
    std::transform(A.begin(), A.end(), A.begin(),
                   [](int a) -> int { return a * a; });
    return A;
  }
  vector<int> sortedSquares(vector<int> &A) {
    int len = A.size();
    int j = 0;
    while (j < len && A[j] <= 0) {
      ++j;
    }
    int i = j - 1, k = 0;
    int si, sj;
    vector<int> ret(A);
    while (i >= 0 && j < len) {
      si = A[i] * A[i];
      sj = A[j] * A[j];
      if (si <= sj) {
        ret[k++] = si;
        --i;
      } else {
        ret[k++] = sj;
        ++j;
      }
    }
    while (i >= 0) {
      ret[k++] = A[i] * A[i];
      --i;
    }
    while (j < len) {
      ret[k++] = A[j] * A[j];
      ++j;
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
  vector<int> A{-7, -3, 2, 3, 11};
  vector<int> ret = sol.sortedSquares(A);
  print_vec(A);
  std::cout << " -> ";
  print_vec(ret);
  std::cout << std::endl;
  return 0;
}