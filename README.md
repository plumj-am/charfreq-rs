# charfreq-rs 🦀

Count the occurrences of characters in a codebase or any directory.

A Rust rewrite of
[https://github.com/plumj-am/char-freq](https://github.com/plumj-am/char-freq).

The original Python implementation was created to determine the symbols I use
most when writing code so I could optimise the layout on my split keyboard.

My first actual project written in Rust outside of learning/exercises so this
was mostly for practice.

If improvements can be made, please open a PR or issue!

## Usage:

### Install

```sh
cargo install charfreq
```

### Run

```
Usage: charfreq [OPTIONS] --dir <REPO_PATH>

Options:
  -d, --dir <REPO_PATH>            Path to the repository
  -t, --top <TOP>                  Number of top characters to display [default: 20]
  -s, --show-spaces                Include spaces and whitespace characters in the output
  -e, --exclude-letters            Exclude all letters (A-Z, a-z) from the output
  -c, --csv                        Save results as CSV in the current working directory
  -v, --verbose                    Show files with errors during the scan (usually invalid file types)
  -i, --ignore <IGNORE_FILETYPES>  Additional filetypes to ignore (comma-separated or once for each filetype)
  -I, --ignore-dir <IGNORE_DIRS>   Additional directories to ignore (comma-separated or once for each directory)
  -h, --help                       Print help
```

Example:

```
$ ./charfreq -d ~/projects/charfreq-rs --top 5 --exclude-letters
```

Will show the top 5 non-alphabetic characters in a codebase.

> [!NOTE] Many filetypes (e.g. `.exe`, `.mp3`) and directories
> (e.g.`node_modules/`, `.idea/`) are ignored by default.

A full list of ignored filetypes and directories can be found in
`src/scanner.rs`.

## Benchmarks

### Test

**Tool**: **[hyperfine](https://github.com/sharkdp/hyperfine)**

**Tested on**:
- Linux kernel source tree: **[torvalds/linux](https://github.com/torvalds/linux)**
- `~82_333` files
- `~1_508_915_498` characters

**Hardware**:

- `i5-13600KF @5.2GHz (OC)`,
- `2x16GB DDR5 G.Skill Z5 Trident @7000MT/s (OC)`,
- `WD 250GB SATA SSD` (generic, cheap model)

```sh
$ hyperfine --warmup=10 --runs=10 --shell=bash \
	'python3 ./char-freq/char_freq.py ./linux' \
	'./charfreq-rs/target/release/charfreq-rs -d ./linux' \
```

^ Compares the latest version to the original Python script.

### Latest results

```
Benchmark 1: python3 ./char-freq/char_freq.py ./linux
  Time (mean ± σ):     39544.7 ms ± 1519.7 ms    [User: 0.0 ms, System: 0.0 ms]
  Range (min … max):   38151.7 ms … 41511.4 ms    10 runs

Benchmark 2: ./charfreq-rs/target/release/charfreq-rs -d ./linux
  Time (mean ± σ):     482.4 ms ±  21.5 ms    [User: 1810.9 ms, System: 3885.3 ms]
  Range (min … max):   456.9 ms … 513.3 ms    10 runs

Summary
  ./charfreq-rs/target/release/charfreq-rs -d ./linux ran
   81.98 ± 4.82 times faster than python3 ./char-freq/char_freq.py ./linux
```

TL;DR: The latest Rust version is ~82x faster than the original Python script.

## Improvements

- Testing
- Push performance further

## License

Copyright (c) PlumJam 2025-now <git@plumj.am>

This project is licensed under the MIT license ([LICENSE] or
<http://opensource.org/licenses/MIT>)

[license]: ./LICENSE
