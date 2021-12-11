#include <gtest/gtest.h>
#include <vector>

using namespace std;

int min_path_sum(vector<vector<int>> &grid) {
  const int m = grid.size();
  const int n = grid[0].size();
  vector<vector<int>> costs(m, vector<int>(n, 0));

  costs[0][0] = grid[0][0];

  for (int i = 1; i < m; i++) {
    costs[i][0] = grid[i][0] + costs[i - 1][0];
  }
  for (int i = 1; i < n; i++) {
    costs[0][i] = grid[0][i] + costs[0][i - 1];
  }

  for (int i = 1; i < m; i++) {
    for (int j = 1; j < n; j++) {
      costs[i][j] = grid[i][j] + min(costs[i - 1][j], costs[i][j - 1]);
    }
  }
  return costs[m - 1][n - 1];
}

TEST(MinPathSum, Test) {
  vector<vector<int>> input = {{1, 3, 1}, {1, 5, 1}, {4, 2, 1}};
  ASSERT_EQ(min_path_sum(input), 7);
}
