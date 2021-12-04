# algo

Coding problems with modern languages.

## Folder structure

- `packages`
  - `algo-go`: solutions in golang
  - `algo-rust`: solutions in rust
  - `algo-cpp`: solutions in c++ 20
  - `algo-ts`: solutions in typescript
  - `algo-py`: solutions in python 3

## Coding Problems

All problems are prefixed with `algo-` an enumerated from zero.
### [algo-00] Most frequently occurring item in array

Given a non-empty array of integers, find the first most frequently occurring item.
E.g. [1,2,1,3,1] -> 1

Input: `arr: int[]`
Output: `int`

#### Solution 

- Use a hash map to keep track of occurrences for each item
- keep track of the item with most occurrences
- Time Complexity: `O(n)`
- Auxiliary space: `O(n)`

## Tests

Run tests for all packages
```sh
make test
```

## Collaboration

If you have some problems to add, please feel free to open a pull request.
