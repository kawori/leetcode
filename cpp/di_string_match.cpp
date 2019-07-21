// Given a string S that only contains "I" (increase) or "D" (decrease),
// let N =S.length. Return any permutation A of [0, 1, ..., N]
// such that for all i = 0,..., N-1:
// If S[i] == "I", then A[i] < A[i+1] If S[i] == "D", then A[i] > A[i+1]

// Example 1:
// Input: "IDID"
// Output: [0,4,1,3,2]

// Example 2:
// Input: "III"
// Output: [0,1,2,3]

// Example 3:
// Input: "DDI"
// Output: [3,2,0,1]

// Note:
// 1 <= S.length <= 10000
// S only contains characters "I" or "D".

#include <iostream>
#include <string>
#include <vector>
using std::string;
using std::vector;

class Solution {
public:
  vector<int> diStringMatch(string S) {
    int N = S.size();
    vector<int> A(N + 1, 0);
    int r = N, l = 0;
    if ('I' == S[0]) {
      A[0] = l++;
    } else {
      A[0] = r--;
    }
    for (int i = 0; i < N - 1; ++i) {
      if ('I' == S[i]) {
        if ('D' == S[i + 1]) {
          A[i + 1] = r--;
        } else {
          A[i + 1] = l++;
        }
      } else {
        if ('I' == S[i + 1]) {
          A[i + 1] = l++;
        } else {
          A[i + 1] = r--;
        }
      }
    }
    if ('I' == S[N - 1]) {
      A[N] = r--;
    } else {
      A[N] = l++;
    }
    return A;
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

    string di("IDID");
    std::cout << di << " -> ";
    print_vec(sol.diStringMatch(di));
    std::cout << "\n";

    di = "III";
    std::cout << di << " -> ";
    print_vec(sol.diStringMatch(di));
    std::cout << "\n";

    di = "DDI";
    std::cout << di << " -> ";
    print_vec(sol.diStringMatch(di));
    std::cout << "\n";

    return 0;
}
