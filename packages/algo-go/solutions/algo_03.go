package solutions

func MaxSubstr(input string) int {
	max := 0
	start := 0
	indexes := [128]int{}
	for i := 0; i < 128; i++ {
		indexes[i] = -1
	}
	for i, c := range input {
		if indexes[byte(c)] >= start {
			start = indexes[byte(c)] + 1
		}
		indexes[uint8(c)] = i
		if i-start+1 > max {
			max = i - start + 1
		}
	}

	return max
}
