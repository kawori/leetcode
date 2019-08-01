// Given two strings s and t,
// write a function to determine if t is an anagram of s.

// Example 1:
// Input: s = "anagram", t = "nagaram"
// Output: true

// Example 2:
// Input: s = "rat", t = "car"
// Output: false

// Note:
// You may assume the string contains only lowercase alphabets.

// Follow up:
// What if the inputs contain unicode characters?
// How would you adapt your solution to such case?

#include <iostream>
#include <string>
using std::string;

class Solution {
public:
  bool isAnagram(string s, string t) {
    if (s.size() != t.size()) {
      return false;
    }
    int counter[26] = {0};
    for (char c : s) {
      counter[c - 'a']++;
    }
    for (char c : t) {
      counter[c - 'a']--;
      if (counter[c - 'a'] < 0) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  Solution sol;
  string s1("anagram"), s2("nagaram");
  std::cout << "\"anagram\", \"nagaram\" -> ";
  if (sol.isAnagram(s1, s2)) {
    std::cout << "true\n";
  } else {
    std::cout << "false\n";
  }

  return 0;
}
