// Given a matrix A, return the transpose of A.
// The transpose of a matrix is the matrix flipped over it's main diagonal,
// switching the row and column indices of the matrix.

// Example 1:
// Input: [[1,2,3],[4,5,6],[7,8,9]]
// Output: [[1,4,7],[2,5,8],[3,6,9]]

// Example 2:
// Input: [[1,2,3],[4,5,6]]
// Output: [[1,4],[2,5],[3,6]]

// Note:
// 1 <= A.length <= 1000
// 1 <= A[0].length <= 1000

#include <iostream>
#include <vector>
using std::vector;

class Solution {
public:
  vector<vector<int>> transpose(vector<vector<int>> &A) {
    int nrow = A.size(), ncol = A[0].size();
    vector<vector<int>> ret(ncol);
    for (int i = 0; i < ncol; ++i) {
      vector<int> row(nrow, 0);
      for (int j = 0; j < nrow; ++j) {
        row[j] = A[j][i];
      }
      ret[i] = row;
    }
    return ret;
  }
  void print(const vector<vector<int>> &A) {
    int nrow = A.size(), ncol = A[0].size();
    std::cout << "\n";
    for (int i = 0; i < nrow; ++i) {
      for (int j = 0; j < ncol; ++j) {
        std::cout << std::right << A[i][j] << " ";
      }
      std::cout << "\n";
    }
    std::cout << "\n";
  }
};

int main() {
  vector<vector<int>> A{{1, 2, 3}, {4, 5, 6}};
  Solution sol;
  sol.print(A);
  auto AT = sol.transpose(A);
  std::cout << "->\n";
  sol.print(AT);

  return 0;
}
