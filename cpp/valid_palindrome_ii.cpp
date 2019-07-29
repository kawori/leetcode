// Given a non-empty string s, you may delete at most one character.
// Judge whether you can make it a palindrome.

// Example 1:
// Input: "aba"
// Output: True

// Example 2:
// Input: "abca"
// Output: True
// Explanation: You could delete the character 'c'.

// Note:
// The string will only contain lowercase characters a-z.
// The maximum length of the string is 50000.

#include <iostream>
#include <string>
using std::string;

class Solution {
public:
  bool validPalindrome(string s) {
    int i = -1, j = s.length();
    while (++i < --j) {
      if (s[i] != s[j]) {
        return isPalindrome(s, i + 1, j) || isPalindrome(s, i, j - 1);
      }
    }
    return true;
  }
  bool isPalindrome(const string &s, int i, int j) {
    while (i < j) {
      if (s[i++] != s[j--]) {
        return false;
      }
    }
    return true;
  }
};

int main() {
    Solution sol;

    string s("aba");
    std::cout << s << " -> ";
    if (sol.validPalindrome(s)) {
        std::cout << "true\n";
    } else {
        std::cout << "false\n";
    }

    s = "abca";
    std::cout << s << " -> ";
    if (sol.validPalindrome(s)) {
        std::cout << "true\n";
    } else {
        std::cout << "false\n";
    }

    return 0;
}
