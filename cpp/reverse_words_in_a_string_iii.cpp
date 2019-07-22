// Given a string, you need to reverse the order of charactersin each word
// within a sentence while still preserving whitespace and initial word order.

// Example 1:
// Input: "Let's take LeetCode contest"
// Output: "s'teL ekat edoCteeL tsetnoc"

// Note: In the string, each word is separated by single space
// and there will not be any extra space in the string.

#include <iostream>
#include <string>
using std::string;

class Solution {
public:
  void reverse(string &s, int beg, int end) {
    int len = end - beg, mid = beg + len / 2;
    for (int i = beg; i < mid; ++i) {
      std::swap(s[i], s[(end - 1) - (i - beg)]);
    }
  }

  string reverseWords(string s) {
    size_t beg = 0, pos = 0;
    do {
      pos = s.find(" ", beg);
      if (pos != string::npos) {
        reverse(s, beg, pos);
        beg = pos + 1;
      }
    } while (pos != string::npos);
    reverse(s, beg, s.size());
    return s;
  }
};

int main() {
  Solution sol;
  string s("Let's take LeetCode contest");
  std::cout << "\"" << s << "\""
            << " -> "
            << "\"" << sol.reverseWords(s) << "\"" << std::endl;
  return 0;
}
