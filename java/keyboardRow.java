// Given a List of words, return the words that can be typed
// using letters of alphabet on only one row's of American keyboard.

// Example:
// Input: ["Hello", "Alaska", "Dad", "Peace"]
// Output: ["Alaska", "Dad"]

// Note:
// You may use one character in the keyboard more than once.
// You may assume the input string will only contain letters of alphabet.

import java.util.List;
import java.util.LinkedList;

class Solution {
    String[] lines = { "qwertyuiop", "asdfghjkl", "zxcvbnm" };

    public boolean contains(String s, char ch) {
        for (int i = 0; i < s.length(); ++i) {
            if (s.charAt(i) == ch) {
                return true;
            }
        }
        return false;
    }

    public int indexOf(char ch) {
        for (int i = 0; i < 3; ++i) {
            if (contains(lines[i], ch)) {
                return i;
            }
        }
        return -1;
    }

    public String[] findWords(String[] words) {
        List<String> ret = new LinkedList<String>();
        for (String w : words) {
            int len = w.length();
            if (2 > len) {
                ret.add(w);
            } else {
                String tmp = w.toLowerCase();
                int index = indexOf(tmp.charAt(0));
                boolean isLine = true;
                for (int i = 1; i < len; ++i) {
                    if (index != indexOf(tmp.charAt(i))) {
                        isLine = false;
                        break;
                    }
                }
                if (isLine) {
                    ret.add(w);
                }
            }
        }
        String[] ss = new String[ret.size()];
        ss = ret.toArray(ss);
        return ss;
    }
}
