// Write a function that reverses a string.
// The input string is given as an array of characters char[].

// Do not allocate extra space for another array,
// you must do this by modifying the input array in-place with O(1) extra memory.
// You may assume all the characters consist of printable ascii characters.

// Example 1:
// Input: ["h","e","l","l","o"]
// Output: ["o","l","l","e","h"]

// Example 2:
// Input: ["H","a","n","n","a","h"]
// Output: ["h","a","n","n","a","H"]

#include <iostream>
#include <vector>
using std::vector;

class Solution {
public:
    void reverseString(vector<char>& s) {
        int len = s.size();
        for (int i = 0; i < len / 2; ++i) {
            std::swap(s[i], s[len - 1 - i]);
        }
    }
};

template <typename T>
void print_vec(const vector<T> &vec) {
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
    vector<char> vec{'h','e','l','l','o'};
    print_vec(vec);
    std::cout << " -> ";
    sol.reverseString(vec);
    print_vec(vec);
    std::cout << std::endl;

    return 0;
}
