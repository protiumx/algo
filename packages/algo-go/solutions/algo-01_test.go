package solutions

import (
	"reflect"
	"testing"
)

var (
	input = [][]byte{
		{'E', 'E', 'E', 'E', 'E'},
		{'E', 'E', 'M', 'E', 'E'},
		{'E', 'E', 'E', 'E', 'E'},
		{'E', 'E', 'E', 'E', 'E'},
	}
	expected = [][]byte{
		{'B', '1', 'E', '1', 'B'},
		{'B', '1', 'M', '1', 'B'},
		{'B', '1', '1', '1', 'B'},
		{'B', 'B', 'B', 'B', 'B'},
	}
)

func TestMineSweeper(t *testing.T) {
	if !reflect.DeepEqual(expected, Minesweeper(input, []int{3, 0})) {
		t.Error("Failed")
	}
}

func TestMineSweeperDFS(t *testing.T) {
	if !reflect.DeepEqual(expected, MinesweeperDFS(input, []int{3, 0})) {
		t.Error("Failed")
	}
}
