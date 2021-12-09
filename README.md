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

## Packages

### `algo-cpp`
- C++ 11/17
- Testing: [gtest](https://github.com/google/googletest)
- Conventions: [Google style guide](https://google.github.io/styleguide/cppguide.html)

### `algo-go`
- Go 1.7
- Testing: `go test`.
- Conventions: [Uber style guide](https://github.com/uber-go/guide)

### `algo-py`
- python 3.x
- Testing: `python unittest`
- Conventions: [google pyguide](https://google.github.io/styleguide/pyguide.html)

### `algo-rust`
- rustc 1.56
- Testing: `cargo test`
- Conventions: `cargo clippy`

### `algo-ts`
- TypeScript 4.5
- Testing: [jest](https://jestjs.io/)
- Conventions: [airbnb](https://github.com/airbnb/javascript) for typescript with [eslint](https://eslint.org/)

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
- Adding more coding problems (with solutions in all packages if possible)
- Improving existing solutions
- Improving projects' config/setup

**NOTE:** This repository follows [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) practices.

### Adding Coding Problems
Ideally what we want:
- A good description with examples, images or a link to `leetcode`, `hackerank` or similar.
- All PR must be submitted with **tests**
- If possible, you might want to provide the solutions in all the different languages.


## TODO
- [ ] Setup linting CI jobs
- [ ] Use yarn pnp to reduce CI run time
- [ ] Add GTest sources to repo
- [ ] Fix type check in vscode for algo-cpp and gtest
- [ ] Setup linting for every package
- [ ] Setup input text files to load during tests
