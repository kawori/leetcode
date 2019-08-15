// Your friend is typing his name into a keyboard.
// Sometimes, when typing a character c, the key might get long pressed, and the
// character will be typed 1 or more times. You examine the typed characters of
// the keyboard. Return True if it is possible that it was your friends name,
// with some characters (possibly none) being long pressed.

// Example 1:
// Input: name = "alex", typed = "aaleex"
// Output: true
// Explanation: 'a' and 'e' in 'alex' were long pressed.

// Example 2:
// Input: name = "saeed", typed = "ssaaedd"
// Output: false
// Explanation: 'e' must have been pressed twice, but it wasn't in the typed
// output.

// Example 3:
// Input: name = "leelee", typed = "lleeelee"
// Output: true

// Example 4:
// Input: name = "laiden", typed = "laiden"
// Output: true
// Explanation: It's not necessary to long press any character.

// Note:
// name.length <= 1000
// typed.length <= 1000
// The characters of name and typed are lowercase letters.

#include <string>
#include <vector>
using std::string;
using std::vector;

class Solution {
public:
  struct Pr {
    char c;
    int count;
    Pr(char c, int count) : c(c), count(count) {}
  };
  vector<Pr> get(const string &s) {
    vector<Pr> ret;
    int i = 0, len = s.size();
    while (i < len) {
      int j = i;
      int sum = 0;
      while (j < len && s[j] == s[i]) {
        j++;
        sum++;
      }
      ret.emplace_back(s[i], sum);
      i = j;
    }
    return ret;
  }
  bool isLongPressedName(string name, string typed) {
    vector<Pr> np = get(name), nt = get(typed);
    if (np.size() != nt.size()) {
      return false;
    }
    int len = np.size();
    for (int i = 0; i < len; ++i) {
      if (np[i].count > nt[i].count) {
        return false;
      }
    }
    return true;
  }
};
