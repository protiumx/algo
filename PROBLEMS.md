# Coding Problems

All problems are enumerated from zero and formatted as `[algo-xx]`.
Difficulty of each problem is annotated in the title as [`easy`|`medium`|`hard`]

### `[algo-00]` Most frequently occurring item in array `[easy]`

Given an array, find the most frequent element in it. If there are multiple elements that appear a maximum number of times, print any one of them.
E.g. [1,2,1,3,1] -> 1

Input: `arr int[], n int`
Output: `int`

#### Solution 

- Use a hash map to keep track of occurrences for each item
- keep track of the item with most occurrences

**Time Complexity:** `O(n)` -> We need to check the whole array
**Auxiliary space:** `O(n)` -> It's possible that the array has all unique numbers

### `[algo-01]` Minesweeper Game `[medium]`

Problem description can be found at [leetcode](https://leetcode.com/problems/minesweeper/)

### Solution
- First check if the cell at the `click` is a mine.
- If `click` position is not a mine, perform DFS from the click position, searching for `empty` cells `"E"`
- If there is a mine in the surroundings of the cell i.e. 1 cell in all 8 directions, stop DFS and set the sell with the number of mines found. If there are no mines, mark the cell with `"B"` and continue DFS

**Time Complexity:** `O(m * n)` -> Worst case we will visit every cell
**Auxiliary space:** `O(m + n)` -> Since we are doing `DFS`, at some point our que could contain all the items from the a cell's row and all the items from a the column.
