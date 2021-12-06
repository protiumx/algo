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
- Time Complexity: `O(n)`
- Auxiliary space: `O(n)`

