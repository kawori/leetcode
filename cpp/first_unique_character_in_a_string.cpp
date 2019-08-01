// Given a string, find the first non-repeating character in it
// and return it's index. If it doesn't exist, return -1.

// Examples:
// s = "leetcode"
// return 0.

// s = "loveleetcode",
// return 2.

// Note: You may assume the string contain only lowercase letters.

#include <iostream>
#include <string>
using std::string;

class Solution {
public:
  int firstUniqChar(string s) {
    int my_map[26] = {0};
    for (char ch : s) {
      my_map[ch - 'a']++;
    }
    int len = s.size();
    for (int i = 0; i < len; ++i) {
      if (1 == my_map[s[i] - 'a']) {
        return i;
      }
    }
    return -1;
  }
};

int main() {
  Solution sol;
  string s("leetcode");
  std::cout << "\"leetcode\" -> " << sol.firstUniqChar(s) << "\n";
  s = "loveleetcode";
  std::cout << "\"loveleetcode\" -> " << sol.firstUniqChar(s) << "\n";

  return 0;
}
