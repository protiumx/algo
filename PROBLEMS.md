# Coding Problems

All problems are enumerated from zero and formatted as `[algo-xx]`.
Difficulty of each problem is annotated in the title as [`easy`|`medium`|`hard`]

## `[algo-00]` Most frequently occurring item in array `[easy]`

Given an array, find the most frequent element in it. If there are multiple elements that appear a maximum number of times, print any one of them.
E.g. [1,2,1,3,1] -> 1

Input: `arr int[], n int`
Output: `int`

### Solution 

- Use a hash map to keep track of occurrences for each item
- keep track of the item with most occurrences

**Time Complexity:** `O(n)` -> We need to check the whole array
**Auxiliary space:** `O(n)` -> It's possible that the array has all unique numbers

## `[algo-01]` Minesweeper Game `[medium]`

Problem description can be found at [leetcode](https://leetcode.com/problems/minesweeper/)

### Solution
- First check if the cell at the `click` is a mine.
- If `click` position is not a mine, perform DFS from the click position, searching for `empty` cells `"E"`
- If there is a mine in the surroundings of the cell i.e. 1 cell in all 8 directions, stop DFS and set the sell with the number of mines found. If there are no mines, mark the cell with `"B"` and continue DFS

**Time Complexity:** `O(m * n)` -> Worst case we will visit every cell
**Auxiliary space:** `O(m + n)` -> Since we are doing `DFS`, at some point our que could contain all the items from the a cell's row and all the items from a the column.

## `[algo-02]` Validate Binary Search Tree `[medium]`

Problem description can be found at [leetcode](https://leetcode.com/problems/validate-binary-search-tree/)

## Solution
- For each node we need to check if expectations are met for the value and the left and right subtree
- All values on the left should be compared to a `max` value.
- All values on the right should be compared to a `min` value.

NOTE: it can also be solved by traversing the tree in-order.

**Time Complexity:** `O(n)` -> We visit all nodes in the tree
**Auxiliary space:** `O(1)` -> The extra space is constant


## `[algo-03]` Longest Substring Without Repeating Characters

Problem description can be found at [leetcode](https://leetcode.com/problems/longest-substring-without-repeating-characters/)

### Solution
- The problem says that the input string can contain letters, **numbers and symbols**. This tell us that we cannot simply use an array[26] for the letters in [a-z]
- When we are iterating the string we are performing 3 operations
	- Counting the amount of chars a substring has
	- Checking if we have seen a repetition
	- Updating the max count

E.g. 
1 - `"abcblkj"`, max = 0, Array[a-z] `indexes` where `indexes[i] = -1; i in [a-z]`
2 - When `i = 3` we have: `index[str[i]] = 1; max = 3; start = 0` -> `start = index[str[i]] + 1 = 2; index[str[i]] = i = 3`
2 - When `i = len(str) -1` we have: `max = 4; start = 2; indexes['j'] = -1`

**Time Complexity:** `O(n)` -> We iterate the whole string
**Auxiliary space:** `O(1)` -> The extra space is the number of characters in `[a-z]`
