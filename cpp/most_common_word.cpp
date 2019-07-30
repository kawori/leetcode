// Given a paragraph and a list of banned words,
// return the most frequent word that is not in the list of banned words.
// It is guaranteed there is at least one word that isn't banned,
// and that the answer is unique.

// Words in the list of banned words are given in lowercase,
// and free of punctuation. Words in the paragraph are not case sensitive.
// The answer is in lowercase.

// Example:
// Input:
// paragraph = "Bob hit a ball, the hit BALL flew far after it was hit."
// banned = ["hit"]
// Output: "ball"
// Explanation:
// "hit" occurs 3 times, but it is a banned word.
// "ball" occurs twice (and no other word does),
// so it is the most frequent non-banned word in the paragraph.
// Note that words in the paragraph are not case sensitive,
// that punctuation is ignored (even if adjacent to words, such as "ball,"),
// and that "hit" isn't the answer
// even though it occurs more because it is banned.

// Note:
// 1 <= paragraph.length <= 1000.
// 0 <= banned.length <= 100.
// 1 <= banned[i].length <= 10.
// The answer is unique, and written in lowercase
// (even if its occurrences in paragraph may have uppercase symbols,
// and even if it is a proper noun.)
// paragraph only consists of letters, spaces, or the punctuation symbols !?',;.
// There are no hyphens or hyphenated words.
// Words only consist of letters,
// never apostrophes or other punctuation symbols.

#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>
using std::map;
using std::set;
using std::string;
using std::vector;

class Solution {
public:
  void replace(string &s) {
    for (char &ch : s) {
      if (ch == '!' || ch == '?' || ch == '\'' || ch == '.' || ch == ';' ||
          ch == ',') {
        ch = ' ';
      }
    }
  }

  void to_lower(string &s) {
    std::transform(
        s.begin(), s.end(), s.begin(),
        [](unsigned char c) -> unsigned char { return std::tolower(c); });
  }

  vector<string> split_by_space(const string &paragraph) {
    int beg = 0, end = 0;
    string word;
    vector<string> ret;
    do {
      word = "";
      end = paragraph.find(" ", beg);
      if (end == string::npos) {
        word = paragraph.substr(beg);
      } else {
        word = paragraph.substr(beg, end - beg);
      }
      if (!word.empty()) {
        ret.push_back(word);
      }
      beg = end + 1;
    } while (end != string::npos);
    return ret;
  }

  string mostCommonWord(string paragraph, vector<string> &banned) {
    set<string> banned_set(banned.begin(), banned.end());
    replace(paragraph);
    to_lower(paragraph);
    vector<string> tokens = split_by_space(paragraph);
    map<string, int> my_map;
    for (string const &str : tokens) {
      if (!banned_set.count(str)) {
        if (my_map.count(str)) {
          my_map[str]++;
        } else {
          my_map[str] = 1;
        }
      }
    }
    int cnt = 0;
    string ret;
    for (auto &elem : my_map) {
      if (elem.second > cnt) {
        cnt = elem.second;
        ret = elem.first;
      }
    }
    return ret;
  }
};

int main() {
  string paragraph("Bob hit a ball, the hit BALL flew far after it was hit.");
  vector<string> banned{"hit"};
  Solution sol;
  std::cout << "paragraph: \"" << paragraph << "\"\n";
  std::cout << "banned: [\"hit\"]\n";
  std::cout << "-> \"" << sol.mostCommonWord(paragraph, banned) << "\"\n";

  return 0;
}