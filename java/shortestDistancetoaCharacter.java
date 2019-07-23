// Given a string S and a character C,
// return an array of integers representing
// the shortest distance from the character C in the string.

// Example 1:
// Input: S = "loveleetcode", C = 'e'
// Output: [3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]

// Note:
// S string length is in [1, 10000].
// C is a single character, and guaranteed to be in string S.
// All letters in S and C are lowercase.

import java.util.ArrayList;

class Solution {
    public int[] shortestToChar(String S, char C) {
        ArrayList<Integer> index = new ArrayList<Integer>();
        int from = -1;
        while (true) {
            from = S.indexOf(C, from + 1);
            if (from == -1) {
                break;
            }
            index.add(from);
        }
        int len = S.length(), occur = index.size();
        int[] ret = new int[len];
        for (int i = 0; i < len; ++i) {
            int j = 0;
            int idx;
            do {
                idx = index.get(j++);
            } while (j < occur && idx < i);
            if (j == occur && idx < i) {
                ret[i] = i - idx;
            } else {
                if (idx == i) {
                    ret[i] = 0;
                } else {
                    if (j < 2) {
                        ret[i] = idx - i;
                    } else {
                        int dl = i - index.get(j - 2);
                        int dr = idx - i;
                        ret[i] = (dl < dr) ? dl : dr;
                    }
                }
            }
        }
        return ret;
    }
}