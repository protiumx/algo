import Queue from "./collections/queue";

const DIRECTIONS = [
  [-1, -1],
  [-1, 0],
  [-1, 1],
  [0, -1],
  [0, 1],
  [1, -1],
  [1, 0],
  [1, 1],
];

export function minesweeper(board: string[][], click: number[]): string[][] {
  const [clickX, clickY] = click;
  if (board[clickX][clickY] === "M") {
    board[clickX][clickY] = "X";
    return board;
  }

  board[clickX][clickY] = "B";
  const m = board.length;
  const n = board[0].length;
  const queue = new Queue<number>();
  queue.enqueue(clickX * n + clickY);

  while (!queue.isEmpty()) {
    const point = queue.dequeue()!;
    const px = (point / n) >> 0; // >> will only take the integral part
    const py = point % n;
    const mines = getAdjacentMines(board, px, py, m, n);
    if (mines > 0) {
      board[px][py] = mines.toString();
    } else {
      let i = 0;
      let j = 0;
      for (const direction of DIRECTIONS) {
        i = px + direction[0];
        j = py + direction[1];
        if (isValidIndex(i, j, m, n) && board[i][j] === "E") {
          board[i][j] = "B";
          queue.enqueue(i * n + j);
        }
      }
    }
  }

  return board;
}

export function minesweeperDFS(board: string[][], click: number[]): string[][] {
  const [clickX, clickY] = click;
  const m = board.length;
  const n = board[0].length;
  if (board[clickX][clickY] === "M") {
    board[clickX][clickY] = "X";
  } else {
    dfs(board, clickX, clickY, m, n);
  }
  return board;
}

function dfs(board: string[][], i: number, j: number, m: number, n: number) {
  if (!isValidIndex(i, j, m, n) || board[i][j] != "E") {
    return;
  }

  const mines = getAdjacentMines(board, i, j, m, n);
  if (mines > 0) {
    board[i][j] = mines.toString();
  } else {
    board[i][j] = "B";
    for (const direction of DIRECTIONS) {
      dfs(board, i + direction[0], j + direction[1], m, n);
    }
  }
}

function getAdjacentMines(
  board: string[][],
  x: number,
  y: number,
  m: number,
  n: number
): number {
  let count = 0;
  let i = 0;
  let j = 0;
  for (const direction of DIRECTIONS) {
    i = x + direction[0];
    j = y + direction[1];
    if (isValidIndex(i, j, m, n) && board[i][j] == "M") {
      count++;
    }
  }
  return count;
}

function isValidIndex(i: number, j: number, m: number, n: number): boolean {
  return 0 <= i && i < m && 0 <= j && j < n;
}
