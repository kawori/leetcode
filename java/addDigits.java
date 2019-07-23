// Given a non-negative integer num,
// repeatedly add all its digits until the result has only one digit.

// Example:

// Input: 38
// Output: 2
// Explanation: The process is like: 3 + 8 = 11, 1 + 1 = 2.
//              Since 2 has only one digit, return it.

// Follow up:
// Could you do it without any loop/recursion in O(1) runtime?

class Solution {
    public int accDigits(int num) {
        int ret = 0;
        while (num != 0) {
            ret += num % 10;
            num /= 10;
        }
        return ret;
    }

    // iteration method
    public int addDigits(int num) {
        while (num > 9) {
            num = accDigits(num);
        }
        return num;
    }

    // O(1) method
    public int addDigits2(int num) {
        if (num > 9) {
            num %= 9;
            if (0 == num) {
                return 9;
            }
        }
        return num;
    }
}
