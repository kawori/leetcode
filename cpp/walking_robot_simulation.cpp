// A robot on an infinite grid starts at point (0, 0) and faces north.
// The robot can receive one of three possible types of commands:
// -2: turn left 90 degrees
// -1: turn right 90 degrees
// 1 <= x <= 9: move forward x units

// Some of the grid squares are obstacles.

// The i-th obstacle is at grid point (obstacles[i][0], obstacles[i][1])

// If the robot would try to move onto them,
// the robot stays on the previous grid square instead
// (but still continues following the rest of the route.)

// Return the square of the maximum Euclidean distance
// that the robot will be from the origin.

// Example 1:
// Input: commands = [4,-1,3], obstacles = []
// Output: 25
// Explanation: robot will go to (3, 4)

// Example 2:
// Input: commands = [4,-1,4,-2,4], obstacles = [[2,4]]
// Output: 65
// Explanation: robot will be stuck at (1, 4)
// before turning left and going to (1, 8)

// Note:
// 0 <= commands.length <= 10000
// 0 <= obstacles.length <= 10000
// -30000 <= obstacle[i][0] <= 30000
// -30000 <= obstacle[i][1] <= 30000
// The answer is guaranteed to be less than 2 ^ 31.

#include <algorithm>
#include <iostream>
#include <set>
#include <utility>
#include <vector>
using std::pair;
using std::set;
using std::vector;

class Solution {
public:
  int robotSim(vector<int> &commands, vector<vector<int>> &obstacles) {
    int dx[4] = {0, 1, 0, -1};
    int dy[4] = {1, 0, -1, 0};
    int x = 0, y = 0, di = 0;

    set<pair<int, int>> obstacleSet;
    for (vector<int> obstacle : obstacles)
      obstacleSet.insert(std::make_pair(obstacle[0], obstacle[1]));

    int ans = 0;
    for (int cmd : commands) {
      if (cmd == -2)
        di = (di + 3) % 4;
      else if (cmd == -1)
        di = (di + 1) % 4;
      else {
        for (int k = 0; k < cmd; ++k) {
          int nx = x + dx[di];
          int ny = y + dy[di];
          if (obstacleSet.find(std::make_pair(nx, ny)) == obstacleSet.end()) {
            x = nx;
            y = ny;
            ans = std::max(ans, x * x + y * y);
          }
        }
      }
    }

    return ans;
  }
};

int main() {
  vector<int> commands{4, -1, 4, -2, 4};
  vector<vector<int>> obstacles{{2, 4}};
  Solution sol;
  std::cout << sol.robotSim(commands, obstacles) << std::endl;

  return 0;
}