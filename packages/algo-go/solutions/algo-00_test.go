package solutions

import "testing"

func TestMostFrequent(t *testing.T) {
	type test struct {
		expected int
		input    []int
	}
	testCases := []test{
		{1, []int{1, 3, 4, 5, 4, 1, 1}},
		{2, []int{2, 3, 4}},
	}

	for _, tt := range testCases {
		if MostFrequent(tt.input) != tt.expected {
			t.Error("Failed")
		}
	}
}
