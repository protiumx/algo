package solutions

import "testing"

func TestMinPathSum(t *testing.T) {
	input := [][]int{
		{1, 3, 1},
		{1, 5, 1},
		{4, 2, 1},
	}

	if MinPathSum(input) != 7 {
		t.Error("Failed")
	}
}
