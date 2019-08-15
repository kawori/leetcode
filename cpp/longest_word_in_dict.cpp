// Given a list of strings words representing an English Dictionary, find the
// longest word in words that can be built one character at a time by other
// words in words. If there is more than one possible answer, return the longest
// word with the smallest lexicographical order. If there is no answer, return
// the empty string.

// Example 1:
// Input:
// words = ["w","wo","wor","worl", "world"]
// Output: "world"
// Explanation:
// The word "world" can be built one character at a time by "w", "wo", "wor",
// and "worl".

// Example 2:

// Input:
// words = ["a", "banana", "app", "appl", "ap", "apply", "apple"]
// Output: "apple"
// Explanation:
// Both "apply" and "apple" can be built from other words in the dictionary.
// However, "apple" is lexicographically smaller than "apply".

// Note:
// All the strings in the input will only contain lowercase letters.
// The length of words will be in the range [1, 1000].
// The length of words[i] will be in the range [1, 30].
// Given a list of strings words representing an English Dictionary, find the
// longest word in words that can be built one character at a time by other
// words in words. If there is more than one possible answer, return the longest
// word with the smallest lexicographical order. If there is no answer, return
// the empty string.

#include <algorithm>
#include <string>

class TrieNode {
public:
  char ch_;
  bool is_tail;
  TrieNode *nexts_[26]{nullptr};
  TrieNode(char c) : ch_(c), is_tail(false) {}
};

class Trie {
public:
  TrieNode *root_;
  Trie() : root_(nullptr) {}
  void add_str(const string &s) {
    if (!root_) {
      root_ = new TrieNode('R');
    }
    TrieNode *p = root_;
    int len = s.size();
    for (int i = 0; i < len; ++i) {
      if (!p->nexts_[s[i] - 'a']) {
        p->nexts_[s[i] - 'a'] = new TrieNode(s[i]);
      }
      p = p->nexts_[s[i] - 'a'];
      if (i == len - 1) {
        p->is_tail = true;
      }
    }
  }
  bool is_built(const string &s) {
    TrieNode *p = root_;
    for (char c : s) {
      p = p->nexts_[c - 'a'];
      if (!p->is_tail) {
        return false;
      }
    }
    return true;
  }
};

class Solution {
public:
  string longestWord(vector<string> &words) {
    Trie t;
    for (const string &s : words) {
      t.add_str(s);
    }
    sort(words.begin(), words.end(),
         [](const string &a, const string &b) { return a.size() > b.size(); });
    int pre_len = 0;
    vector<string> ret;
    for (const string &s : words) {
      if (t.is_built(s)) {
        if (s.size() < pre_len) {
          break;
        } else {
          ret.push_back(s);
        }
        pre_len = s.size();
      }
    }
    sort(ret.begin(), ret.end());
    return ret.front();
  }
};
