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

Coding problems with modern languages, TDD and CI.

## Dependencies

- `Cargo`
- `CMake`
- `Go`
- `Python3`
- `Yarn`

## Folder structure

- `packages/`
  - `algo-cpp/`: solutions in c++ (17)
  - `algo-go/`: solutions in golang
  - `algo-py/`: solutions in python 3
  - `algo-rust/`: solutions in rust
  - `algo-ts/`: solutions in typescript

## Coding Problems

All problems are prefixed with `algo-` an enumerated from zero. This way you can easily find the solutions in any package.
Checkout the list of problems [here](PROBLEMS.md)

## Testing

All packages are configured to use `Makefile` as follow
```bash
make -C packages/algo-[lang]/ test
```
Except for `algo-ts`, where we use yarn
```bash
yarn --cwd packages/algo-ts/ test
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
