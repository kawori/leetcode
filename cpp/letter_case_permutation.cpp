// Given a string S, we can transform every letter individually to be lowercase
// or uppercase to create another string. Return a list of all possible strings
// we could create.

// Examples:
// Input: S = "a1b2"
// Output: ["a1b2", "a1B2", "A1b2", "A1B2"]

// Input: S = "3z4"
// Output: ["3z4", "3Z4"]

// Input: S = "12345"
// Output: ["12345"]

// Note:
// S will be a string with length between 1 and 12.
// S will consist only of letters or digits.

#include <string>
#include <vector>
using std::string;
using std::vector;

class Solution {
private:
  int len_;

public:
  void dfs(int i, string &s, vector<string> &ret) {
    if (i == len_) {
      ret.push_back(s);
      return;
    }
    if ((s[i] >= 'a' && s[i] <= 'z') || (s[i] >= 'A' && s[i] <= 'Z')) {
      s[i] ^= 0x20;
      dfs(i + 1, s, ret);
      s[i] ^= 0x20;
    }
    dfs(i + 1, s, ret);
  }
  vector<string> letterCasePermutation(string S) {
    vector<string> ret;
    if (S.empty()) {
      return ret;
    }
    len_ = S.size();
    dfs(0, S, ret);
    return ret;
  }
};