// Given an array of characters, compress it in-place.

// The length after compression must always be
// smaller than or equal to the original array.

// Every element of the array should be a character (not int) of length 1.

// After you are done modifying the input array in-place,
// return the new length of the array.

// Follow up:
// Could you solve it using only O(1) extra space?

// Example 1:
// Input:
// ["a","a","b","b","c","c","c"]
// Output:
// Return 6, and the first 6 characters of the input array should be:
// ["a","2","b","2","c","3"]
// Explanation: "aa" is replaced by "a2".
// "bb" is replaced by "b2". "ccc" is replaced by "c3".

// Example 2:
// Input:
// ["a"]
// Output:
// Return 1, and the first 1 characters of the input array should be: ["a"]
// Explanation:
// Nothing is replaced.

// Example 3:
// Input:
// ["a","b","b","b","b","b","b","b","b","b","b","b","b"]
// Output:
// Return 4, and the first 4 characters of the input array should be:
// ["a","b","1","2"].
// Explanation:
// Since the character "a" does not repeat, it is not compressed.
// "bbbbbbbbbbbb" is replaced by "b12".
// Notice each digit has it's own entry in the array.

// Note:
// All characters have an ASCII value in [35, 126].
// 1 <= len(chars) <= 1000.

#include <iostream>
#include <string>
#include <vector>
using std::string;
using std::vector;

class Solution {
public:
  int compress(vector<char> &chars) {
    int len = chars.size();
    if (2 > len) {
      return len;
    }
    char ch = chars[0];
    int n = 0, beg = 0;
    for (int i = 0; i < len;) {
      if (chars[i] == ch && len - 1 != i) {
        n++;
        i++;
      } else {
        if (chars[i] == ch) {
          n++;
        }
        bool end = false;
        if (len - 1 == i && chars[i] == ch) {
          end = true;
        }
        chars[beg] = ch;
        if (1 < n) {
          string tmp = std::to_string(n);
          int tmp_len = tmp.size();
          for (int j = 0; j < tmp_len; ++j) {
            chars[beg + 1 + j] = tmp[j];
          }
          beg += tmp_len + 1;
        } else {
          beg++;
        }
        if (end) {
          break;
        }
        n = 0;
        ch = chars[i];
      }
    }
    return beg;
  }
};

vector<char> as_chars(const string &str) {
  vector<char> ret(str.begin(), str.end());
  return ret;
}

string to_str(const vector<char> &chars) {
  string str(chars.begin(), chars.end());
  return str;
}

int main() {
  Solution sol;

  vector<char> chars = as_chars("aabbccc");
  std::cout << "\"aabbccc\" -> " << sol.compress(chars);
  std::cout << " -> \"" << to_str(chars) << "\"\n";

  chars = as_chars("abc");
  std::cout << "\"abc\" -> " << sol.compress(chars);
  std::cout << " -> \"" << to_str(chars) << "\"\n";

  chars = as_chars("abbbbbbbbbbbb");
  std::cout << "\"abbbbbbbbbbbb\" -> " << sol.compress(chars);
  std::cout << " -> \"" << to_str(chars) << "\"\n";

  return 0;
}