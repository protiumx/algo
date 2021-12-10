package solutions

import "testing"

func TestMaxSubstr(t *testing.T) {
	input := "abcbfghb"
	if MaxSubstr(input) != 5 {
		t.Error("Failed")
	}
}
