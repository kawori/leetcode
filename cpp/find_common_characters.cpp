// Given an array A of strings made only from lowercase letters, return a list
// of all characters that show up in all strings within the list (including
// duplicates).  For example, if a character occurs 3 times in all strings but
// not 4 times, you need to include that character three times in the final
// answer.

// You may return the answer in any order.

// Example 1:
// Input: ["bella","label","roller"]
// Output: ["e","l","l"]

// Example 2:
// Input: ["cool","lock","cook"]
// Output: ["c","o"]

// Note:
// 1 <= A.length <= 100
// 1 <= A[i].length <= 100
// A[i][j] is a lowercase letter

#include <string>
#include <vector>
#include <iostream>
using std::string;
using std::vector;

class Solution {
public:
  vector<int> chars(const string &s) {
    vector<int> ret(26, 0);
    for (char ch : s) {
      ret[ch - 'a']++;
    }
    return ret;
  }
  vector<string> commonChars(vector<string> &A) {
    int len = A.size();
    vector<int> cnt(26, 100);
    for (const string &s : A) {
      vector<int> tmp = chars(s);
      for (int j = 0; j < 26; ++j) {
        if (tmp[j] < cnt[j]) {
          cnt[j] = tmp[j];
        }
      }
    }
    vector<string> ret;
    for (int i = 0; i < 26; ++i) {
      if (cnt[i] > 0) {
        ret.insert(ret.end(), cnt[i], string(1, 'a' + i));
      }
    }
    return ret;
  }
};

int main() {
    vector<string> A{"bella","label","roller"};
    Solution sol;
    vector<string> ret = sol.commonChars(A);
    for (const string &s : ret) {
        std::cout << s << " ";
    }
    std::cout << "\n";
    return 0;
}
