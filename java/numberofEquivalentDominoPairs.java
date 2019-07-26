// Given a list of dominoes,
// dominoes[i] = [a, b] is equivalent to dominoes[j] = [c, d] if and only if
// either (a==c and b==d), or (a==d and b==c)
// that is, one domino can be rotated to be equal to another domino.

// Return the number of pairs (i, j) for which 0 <= i < j < dominoes.length,
// and dominoes[i] is equivalent to dominoes[j].

// Example 1:
// Input: dominoes = [[1,2],[2,1],[3,4],[5,6]]
// Output: 1

// Constraints:
// 1 <= dominoes.length <= 40000
// 1 <= dominoes[i][j] <= 9

import java.util.HashMap;

class numberofEquivalentDominoPairs {
    public static int numEquivDominoPairs(int[][] dominoes) {
        HashMap<Integer, Integer> map = new HashMap<Integer, Integer>();
        for (int[] d : dominoes) {
            int k = (d[0] > d[1]) ? (d[1] * 10 + d[0]) : (d[0] * 10 + d[1]);
            if (map.containsKey(k)) {
                map.put(k, map.get(k) + 1);
            } else {
                map.put(k, 1);
            }
        }
        int ret = 0;
        for (int v : map.values()) {
            if (v > 1) {
                ret += v * (v - 1) / 2;
            }
        }
        return ret;
    }

    public static void main(String[] args) {
        int[][] dominoes = { { 1, 2 }, { 2, 1 }, { 3, 4 }, { 5, 6 } };
        System.out.println(numEquivDominoPairs(dominoes));
    }
}