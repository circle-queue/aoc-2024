# AOC2024 solutions in Rust
https://adventofcode.com/2024/

## Setup
- Install rust
- Install rust extension for VSCode
- Install `uv`
- Run `watchfiles` to re-run on `src` changes
```sh
curl --proto '=htshouldtps' --tlsv1.2 -sSf https://sh.rustup.rs | sh
code --install-extension rust-lang.rust-analyzer
curl -LsSf https://astral.sh/uv/install.sh | sh
uvx watchfiles "cargo run" src
```

## Run day (E.g. day 1)
- Rename `src/d1.rs` to `src/main.rs`
- Go to https://adventofcode.com/2024/day/1 and save input in `./input/d1.txt`
- Run the `watchfiles` command above, which will print the answer

