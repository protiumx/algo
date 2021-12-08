use std::char;
use std::collections::VecDeque;

const DIRECTIONS: &[[i32; 2]; 8] = &[
  [-1, -1],
  [-1, 0],
  [-1, 1],
  [0, -1],
  [0, 1],
  [1, -1],
  [1, 0],
  [1, 1],
];

fn minesweeper(board: &mut Vec<Vec<char>>, click: Vec<i32>) -> &Vec<Vec<char>> {
  let (click_x, click_y) = (click[0], click[1]);
  if board[click_x as usize][click_y as usize] == 'M' {
    board[click_x as usize][click_y as usize] = 'X';
    return board;
  }
  let (m, n) = (board.len(), board[0].len());
  board[click_x as usize][click_y as usize] = 'B';
  let mut q: VecDeque<i32> = VecDeque::with_capacity(m * n);
  q.push_back(click_x * n as i32 + click_y);

  while !q.is_empty() {
    let point = q.pop_front().unwrap();
    let (px, py) = (point / n as i32, point % n as i32);
    let mines = get_adjacent_mines(board, px as i32, py as i32, m, n);
    if mines > 0 {
      board[px as usize][py as usize] = (mines as u8 + b'0') as char;
    } else {
      for &direction in DIRECTIONS {
        let i = px + direction[0];
        let j = py + direction[1];
        if is_valid_index(i as i32, j as i32, m, n) && board[i as usize][j as usize] == 'E' {
          board[i as usize][j as usize] = 'B';
          q.push_back(i * n as i32 + j);
        }
      }
    }
  }

  board
}

fn minesweeper_dfs(board: &mut Vec<Vec<char>>, click: Vec<i32>) -> &Vec<Vec<char>> {
  let (click_x, click_y) = (click[0], click[1]);
  let (m, n) = (board.len(), board[0].len());

  if board[click_x as usize][click_y as usize] == 'M' {
    board[click_x as usize][click_y as usize] = 'X';
  } else {
    dfs(board, click_x, click_y, m, n);
  }
  board
}

fn dfs(board: &mut Vec<Vec<char>>, i: i32, j: i32, m: usize, n: usize) {
  if !is_valid_index(i, j, m, n) || board[i as usize][j as usize] != 'E' {
    return;
  }
  let mines = get_adjacent_mines(board, i, j, m, n);
  if mines > 0 {
    board[i as usize][j as usize] = (mines as u8 + b'0') as char;
  } else {
    board[i as usize][j as usize] = 'B';
    for &direction in DIRECTIONS {
      dfs(board, i + direction[0], j + direction[1], m, n);
    }
  }
}

fn get_adjacent_mines(board: &[Vec<char>], x: i32, y: i32, m: usize, n: usize) -> i32 {
  let mut count: i32 = 0;
  for &direction in DIRECTIONS {
    let i = x + direction[0];
    let j = y + direction[1];
    if is_valid_index(i, j, m, n) && board[i as usize][j as usize] == 'M' {
      count += 1;
    }
  }
  count
}

fn is_valid_index(i: i32, j: i32, m: usize, n: usize) -> bool {
  0 <= i && i < m as i32 && 0 <= j && j < n as i32
}

#[cfg(test)]
mod test {
  use super::{minesweeper, minesweeper_dfs};

  #[test]
  fn test_minesweeper() {
    let mut input: Vec<Vec<char>> = vec![
      vec!['E', 'E', 'E', 'E', 'E'],
      vec!['E', 'E', 'M', 'E', 'E'],
      vec!['E', 'E', 'E', 'E', 'E'],
      vec!['E', 'E', 'E', 'E', 'E'],
    ];

    let expected: Vec<Vec<char>> = vec![
      vec!['B', '1', 'E', '1', 'B'],
      vec!['B', '1', 'M', '1', 'B'],
      vec!['B', '1', '1', '1', 'B'],
      vec!['B', 'B', 'B', 'B', 'B'],
    ];

    assert_eq!(minesweeper(&mut input, vec![3, 0])[..], expected[..]);
  }

  #[test]
  fn test_minesweeper_dfs() {
    let mut input: Vec<Vec<char>> = vec![
      vec!['E', 'E', 'E', 'E', 'E'],
      vec!['E', 'E', 'M', 'E', 'E'],
      vec!['E', 'E', 'E', 'E', 'E'],
      vec!['E', 'E', 'E', 'E', 'E'],
    ];

    let expected: Vec<Vec<char>> = vec![
      vec!['B', '1', 'E', '1', 'B'],
      vec!['B', '1', 'M', '1', 'B'],
      vec!['B', '1', '1', '1', 'B'],
      vec!['B', 'B', 'B', 'B', 'B'],
    ];

    assert_eq!(minesweeper_dfs(&mut input, vec![3, 0])[..], expected[..]);
  }
}
