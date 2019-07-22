# The Fibonacci numbers, commonly denoted F(n) form a sequence, called the Fibonacci sequence,
# such that each number is the sum of the two preceding ones, starting from 0 and 1.
# That is,
# F(0) = 0, F(1) = 1
# F(N) = F(N - 1) + F(N - 2), for N > 1.

# Given N, calculate F(N).

# Example 1:
# Input: 2
# Output: 1
# Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.

# Example 2:
# Input: 3
# Output: 2
# Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.

# Example 3:
# Input: 4
# Output: 3
# Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.

# Note:
# 0 ≤ N ≤ 30.

class Solution:
    # recursion method
    def fib1(self, N: int) -> int:
        if N < 3:
            return N
        return self.fib1(N - 1) + self.fib1(N - 2)

    # iteration method
    def fib(self, N: int) -> int:
        a = 0
        b = 1
        for _ in range(N):
            a, b = b, a + b
        return a

sol = Solution()
print("F(2) = {}".format(sol.fib(2)))
print("F(3) = {}".format(sol.fib(3)))
print("F(4) = {}".format(sol.fib(4)))
