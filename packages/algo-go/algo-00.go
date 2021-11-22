package main

func MostFrequent(arr []int) int {
	var ret int
	// Track the the max number of occurrences for an item
	maxCount := -1
	counter := make(map[int]int, len(arr))
	for _, n := range arr {
		if _, ok := counter[n]; !ok {
			counter[n] = 1
		} else {
			counter[n]++
		}

		if counter[n] > maxCount {
			maxCount = counter[n]
			ret = n
		}
	}
	return ret
}
