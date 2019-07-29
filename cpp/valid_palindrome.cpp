// Given a string, determine if it is a palindrome,
// considering only alphanumeric characters and ignoring cases.

// Note: For the purpose of this problem,
// we define empty string as valid palindrome.

// Example 1:
// Input: "A man, a plan, a canal: Panama"
// Output: true

// Example 2:
// Input: "race a car"
// Output: false

#include <algorithm>
#include <cctype>
#include <iostream>
#include <string>
using std::string;

class Solution {
public:
  bool is_digit(char a) { return a >= '0' && a <= '9'; }
  bool is_letter(char a) { return a >= 'a' && a <= 'z'; }
  bool is_others(char a) { return !is_digit(a) && !is_letter(a); }
  bool equals(char a, char b) {
    if ((is_digit(a) && is_digit(b)) || (is_letter(a) && is_letter(b))) {
      return a == b;
    }
    return false;
  }
  bool isPalindrome(string s) {
    string sc(s);
    std::transform(
        sc.begin(), sc.end(), sc.begin(),
        [](unsigned char c) -> unsigned char { return std::tolower(c); });
    int i = 0, j = sc.size() - 1;
    while (i < j) {
      if (is_others(sc[i])) {
        ++i;
        continue;
      }
      if (is_others(sc[j])) {
        --j;
        continue;
      }
      if (!equals(sc[i], sc[j])) {
        return false;
      }
      ++i;
      --j;
    }
    return true;
  }
};

int main() {
    string s("A man, a plan, a canal: Panama");
    Solution sol;
    std::cout << s << " -> ";
    if (sol.isPalindrome(s)) {
        std::cout << "true\n";
    } else {
        std::cout << "false\n";
    }

    s = "race a car";
    std::cout << s << " -> ";
    if (sol.isPalindrome(s)) {
        std::cout << "true\n";
    } else {
        std::cout << "false\n";
    }

    return 0;
}
