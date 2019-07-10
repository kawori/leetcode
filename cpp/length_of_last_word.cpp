// Given a string s consists of upper/lower-case alphabets and empty space
// characters ' ', return the length of last word in the string.

// If the last word does not exist, return 0.

// Note: A word is defined as a character sequence consists of non-space
// characters only.

// Example:

// Input: "Hello World"
// Output: 5

#include <string>
#include <cstdio>
using std::string;

class Solution {
public:
  int lengthOfLastWord(string s) {
    int len = s.size(), i = len - 1;
    while (i >= 0 && s[i] == ' ') {
      i--;
    }
    int j = i;
    while (j >= 0 && s[j] != ' ') {
      j--;
    }
    return i - j;
  }
};

int main() {
    string s("Hello World");
    Solution sol;
    std::printf("%s -> %d\n", s.c_str(), sol.lengthOfLastWord(s));
    return 0;
}