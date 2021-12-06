# algo

<p align="left">
    <a href="https://github.com/protiumx/algo/actions/workflows/algo-cpp.yml" alt="algo-cpp">
        <img src="https://github.com/protiumx/algo/actions/workflows/algo-cpp.yml/badge.svg?branch=main"/>
    </a>
    <a href="https://github.com/protiumx/algo/actions/workflows/algo-go.yml" alt="algo-ui">
        <img src="https://github.com/protiumx/algo/actions/workflows/algo-go.yml/badge.svg?branch=main"/>
    </a>
    <a href="https://github.com/protiumx/algo/actions/workflows/algo-py.yml" alt="algo-ui">
        <img src="https://github.com/protiumx/algo/actions/workflows/algo-py.yml/badge.svg?branch=main"/>
    </a>
    <a href="https://github.com/protiumx/algo/actions/workflows/algo-rust.yml" alt="algo-ui">
        <img src="https://github.com/protiumx/algo/actions/workflows/algo-rust.yml/badge.svg?branch=main"/>
    </a>
    <a href="https://github.com/protiumx/algo/actions/workflows/algo-ts.yml" alt="algo-ui">
        <img src="https://github.com/protiumx/algo/actions/workflows/algo-ts.yml/badge.svg?branch=main"/>
    </a>
</p>

Coding problems with modern languages.

## Dependencies

- `Cargo`
- `CMake`
- `Go`
- `Python3`
- `Yarn`

## Folder structure

- `packages`
  - `algo-cpp`: solutions in c++ (17)
  - `algo-go`: solutions in golang
  - `algo-py`: solutions in python 3
  - `algo-rust`: solutions in rust
  - `algo-ts`: solutions in typescript

## Coding Problems

All problems are prefixed with `algo-` an enumerated from zero. This way you can easily find the solutions in any package.
### [algo-00] Most frequently occurring item in array

Given an array, find the most frequent element in it. If there are multiple elements that appear a maximum number of times, print any one of them.
E.g. [1,2,1,3,1] -> 1

Input: `arr int[], n int`
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

## Contributing

Please feel free to create a `PR` for:
- Adding more coding problems
- Improving existing solutions
- Improving projects' config/setup

### Adding Coding Problems
Ideally what we want:
- A good description with examples, images or a link to `leetcode`, `hackerank` or similar.
- All PR must be submitted with **tests**
- If possible, you might want to provide the solutions in all the different languages.
