#include <vector>
#include <queue>
#include <gtest/gtest.h>

using namespace std;

const int DIRECTIONS[8][2] = {
    {-1, -1},
    {-1, 0},
    {-1, 1},
    {0, -1},
    {0, 1},
    {1, -1},
    {1, 0},
    {1, 1},
};

bool is_valid_index(int i, int j, int m, int n);
int get_adjacent_mines(vector<vector<char> > &board, int i, int j, int m, int n);
void dfs(vector<vector<char> > &board, int i, int j, int m, int n);

vector<vector<char> > minesweeper(vector<vector<char> > &board, vector<int> &click)
{
  int clickX = click[0], clickY = click[1];

  if (board[clickX][clickY] == 'M')
  {
    board[clickX][clickY] = 'X';
    return board;
  }

  board[clickX][clickY] = 'B';
  int m = board.size(), n = board[0].size();
  queue<int> q;
  q.push(clickX * n + clickY);

  while (!q.empty())
  {
    int point = q.front();
    q.pop();
    int px = point / n, py = point % n;
    int mines = get_adjacent_mines(board, px, py, m, n);
    if (mines > 0)
    {
      board[px][py] = mines + '0';
    }
    else
    {
      for (int i = 0; i < 8; i++)
      {
        int x = px + DIRECTIONS[i][0];
        int y = py + DIRECTIONS[i][1];
        if (is_valid_index(x, y, m, n) && board[x][y] == 'E')
        {
          board[x][y] = 'B';
          q.push(x * n + y);
        }
      }
    }
  }

  return board;
}

vector<vector<char> > minesweeper_dfs(vector<vector<char> > &board, vector<int> &click)
{
  int clickX = click[0], clickY = click[1];
  int m = board.size(), n = board[0].size();
  if (board[clickX][clickY] == 'M')
  {
    board[clickX][clickY] = 'X';
  }
  else
  {
    dfs(board, clickX, clickY, m, n);
  }
  return board;
}

void dfs(vector<vector<char> > &board, int i, int j, int m, int n)
{
  if (!is_valid_index(i, j, m, n) || board[i][j] != 'E')
  {
    return;
  }

  int mines = get_adjacent_mines(board, i, j, m, n);
  if (mines > 0)
  {
    board[i][j] = mines + '0';
  }
  else
  {
    board[i][j] = 'B';
    for (int d = 0; d < 8; d++)
    {
      dfs(board, i + DIRECTIONS[d][0], j + DIRECTIONS[d][1], m, n);
    }
  }
}

bool is_valid_index(int i, int j, int m, int n)
{
  return 0 <= i && i < m && 0 <= j && j < n;
}

int get_adjacent_mines(vector<vector<char> > &board, int i, int j, int m, int n)
{
  int count = 0;
  for (int d = 0; d < 8; d++)
  {
    int x = i + DIRECTIONS[d][0];
    int y = j + DIRECTIONS[d][1];
    if (is_valid_index(x, y, m, n) && board[x][y] == 'M')
    {
      count++;
    }
  }
  return count;
}

TEST(Minesweeper, UpdatesBoard)
{
  vector<vector<char> > input{
      {'E', 'E', 'E', 'E', 'E'},
      {'E', 'E', 'M', 'E', 'E'},
      {'E', 'E', 'E', 'E', 'E'},
      {'E', 'E', 'E', 'E', 'E'}};
  vector<vector<char> > expected{
      {'B', '1', 'E', '1', 'B'},
      {'B', '1', 'M', '1', 'B'},
      {'B', '1', '1', '1', 'B'},
      {'B', 'B', 'B', 'B', 'B'}};

  vector<int> click{3, 0};
  minesweeper(input, click);
  for (int i = 0; i < 4; i++)
  {
    for (int j = 0; j < 5; j++)
    {
      EXPECT_EQ(input[i][j], expected[i][j]);
    }
  }
}

TEST(MinesweeperDFS, UpdatesBoard)
{
  vector<vector<char> > input{
      {'E', 'E', 'E', 'E', 'E'},
      {'E', 'E', 'M', 'E', 'E'},
      {'E', 'E', 'E', 'E', 'E'},
      {'E', 'E', 'E', 'E', 'E'}};
  vector<vector<char> > expected{
      {'B', '1', 'E', '1', 'B'},
      {'B', '1', 'M', '1', 'B'},
      {'B', '1', '1', '1', 'B'},
      {'B', 'B', 'B', 'B', 'B'}};

  vector<int> click{3, 0};
  minesweeper_dfs(input, click);
  for (int i = 0; i < 4; i++)
  {
    for (int j = 0; j < 5; j++)
    {
      EXPECT_EQ(input[i][j], expected[i][j]);
    }
  }
}
