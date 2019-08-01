// Given a string s and a non-empty string p,
// find all the start indices of p's anagrams in s.

// Strings consists of lowercase English letters only
// and the length of both strings s and p will not be larger than 20,100.
// The order of output does not matter.

// Example 1:
// Input:
// s: "cbaebabacd" p: "abc"
// Output:
// [0, 6]
// Explanation:
// The substring with start index = 0 is "cba", which is an anagram of "abc".
// The substring with start index = 6 is "bac", which is an anagram of "abc".

// Example 2:
// Input:
// s: "abab" p: "ab"
// Output:
// [0, 1, 2]
// Explanation:
// The substring with start index = 0 is "ab", which is an anagram of "ab".
// The substring with start index = 1 is "ba", which is an anagram of "ab".
// The substring with start index = 2 is "ab", which is an anagram of "ab".

#include <iostream>
#include <string>
#include <vector>
using std::string;
using std::vector;

class Solution {
public:
  vector<int> freq(const string &s) {
    vector<int> ret(26, 0);
    for (char c : s) {
      ret[c - 'a']++;
    }
    return ret;
  }
  vector<int> findAnagrams(string s, string p) {
    int len1 = s.size(), len2 = p.size();
    vector<int> map2 = freq(p);
    vector<int> map1 = freq(s.substr(0, len2));
    vector<int> ret;
    if (map1 == map2) {
      ret.push_back(0);
    }
    for (int i = 1; i <= len1 - len2; ++i) {
      map1[s[i - 1] - 'a']--;
      map1[s[i + len2 - 1] - 'a']++;
      if (map1 == map2) {
        ret.push_back(i);
      }
    }
    return ret;
  }
};

template <typename T> void print_vec(const vector<T> &vec) {
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
  string s("cbaebabacd"), p("abc");
  std::cout << "s: \"" << s << "\", p: \"" << p << "\" -> ";
  print_vec(sol.findAnagrams(s, p));
  std::cout << std::endl;

  return 0;
}
