package solutions

import "container/list"

var DIRECTIONS = [][]int{
	{-1, -1},
	{-1, 0},
	{-1, 1},
	{0, -1},
	{0, 1},
	{1, -1},
	{1, 0},
	{1, 1},
}

func Minesweeper(
	board [][]byte,
	click []int,
) [][]byte {
	clickX, clickY := click[0], click[1]
	if board[clickX][clickY] == 'M' {
		board[clickX][clickY] = 'X'
		return board
	}

	board[clickX][clickY] = 'B'
	m, n := len(board), len(board[0])
	queue := list.New()
	queue.PushBack(clickX*n + clickY)

	var i, j int
	for queue.Len() > 0 {
		point := queue.Front()
		queue.Remove(point)
		px, py := point.Value.(int)/n, point.Value.(int)%n
		mines := getAdjacentMines(board, px, py, m, n)
		if mines > 0 {
			board[px][py] = byte(mines + '0')
		} else {
			for _, direction := range DIRECTIONS {
				i, j = px+direction[0], py+direction[1]
				if isValidIndex(i, j, m, n) && board[i][j] == 'E' {
					board[i][j] = 'B'
					queue.PushBack(i*n + j)
				}
			}
		}
	}
	return board
}

func MinesweeperDFS(
	board [][]byte,
	click []int,
) [][]byte {
	clickX, clickY := click[0], click[1]
	m, n := len(board), len(board[0])
	if board[clickX][clickY] == 'M' {
		board[clickX][clickY] = 'X'
	} else {
		dfs(board, clickX, clickY, m, n)
	}
	return board
}

func dfs(board [][]byte, i, j, m, n int) {
	if !isValidIndex(i, j, m, n) || board[i][j] != 'E' {
		return
	}

	mines := getAdjacentMines(board, i, j, m, n)
	if mines > 0 {
		board[i][j] = byte(mines + '0')
	} else {
		board[i][j] = 'B'
		for _, direction := range DIRECTIONS {
			dfs(board, i+direction[0], j+direction[1], m, n)
		}
	}
}

func getAdjacentMines(board [][]byte, x, y, m, n int) int {
	count := 0
	var i, j int
	for _, direction := range DIRECTIONS {
		i, j = x+direction[0], y+direction[1]
		if isValidIndex(i, j, m, n) && board[i][j] == 'M' {
			count++
		}
	}
	return count
}

func isValidIndex(i, j, m, n int) bool {
	return 0 <= i && i < m && 0 <= j && j < n
}
