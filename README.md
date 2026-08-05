# charfreq-rs 🦀

Count the occurrences of characters in a codebase or any directory.

A Rust rewrite of
[https://github.com/plumj-am/char-freq](https://github.com/plumj-am/char-freq).

The original Python implementation was created to determine the symbols I use
most when writing code so I could optimise the layout on my split keyboard.

If improvements can be made, please open a PR or issue!

## Usage:

### Install

```sh
cargo install charfreq
```

### Run

```
Usage: charfreq [OPTION]... REPO_PATH

Arguments:
  REPO_PATH  Path to the repository

Options:
  -t, --top=TOP                            Number of top characters to display [default = 20]
  -s, --show-spaces                        Include spaces and whitespace characters in the output
  -e, --exclude-letters                    Exclude all letters from the output
  -s, --save-csv                           Save results as a CSV in the current directory
  -v, --verbose                            Show files with errors during the scan
  -i, --ignore-filetypes=IGNORE_FILETYPES  Additional filetypes to ignore (repeatable)
  -I, --ignore-dirs=IGNORE_DIRS            Additional directories to ignore (repeatable)
  -h, --help                               display this help and exit
  -V, --version                            output version information and exit
```

Example:

```
charfreq ~/projects/linux --top 5 --exclude-letters
```

Will show the top 5 non-[a-zA-Z] characters in a codebase:

```
Scanning repository: /home/jam/projects/linux

Processed 94787 files
Total characters: 1615894607
Scan time: 0.14s

Top character frequencies:
Char | Count | Percentage
------------------------------
   _ | 101730581 |   6.30%
   0 | 41432453 |   2.56%
   , | 16120619 |   1.00%
   1 | 12666396 |   0.78%
   ; | 12600063 |   0.78%
```

> [!NOTE]
> Many filetypes (e.g. `.exe`, `.mp3`) and directories (e.g.`node_modules/`,
> `.idea/`) are ignored by default.

A full list of ignored filetypes and directories can be found in
`src/scanner.rs`.

## Benchmarks

### Test

**Tool**: **[hyperfine](https://github.com/sharkdp/hyperfine)**

**Tested on**:

- Linux kernel source tree:
  **[torvalds/linux](https://github.com/torvalds/linux)** @
  **[075b7484](https://github.com/torvalds/linux/commit/075b7484)**
- `94_787` files
- `1_615_894_607` characters

**Hardware**:

| Component   | Name                                          |
| ----------- | --------------------------------------------- |
| CPU         | i5-13600KF @5.3GHz (OC)                       |
| RAM         | 2x16GB DDR5 G.Skill Z5 Trident @7000MT/s (OC) |
| Motherboard | Gigabyte Z790 AORUS ELITE AX                  |
| SSD         | Kingston SKC3000S1024G NVME SSD               |
| OS          | NixOS 26.11 (Zokor) x86_64                    |
| Kernel      | Linux 6.18.38                                 |

**Command**:

```
hyperfine --warmup=10 --runs=10 --shell=NONE \
  'python3 ./char_freq.py ~/projects/linux' \
  './charfreq-rs ~/projects/linux'
```

<sup>_↑ compares the latest version to the original Python script_</sup>

### Latest results

```sh
Benchmark 1: ./charfreq-rs ~/projects/linux
  Time (mean ± σ):     148.5 ms ±   4.6 ms    [User: 1328.9 ms, System: 653.9 ms]
  Range (min … max):   138.8 ms … 155.4 ms    10 runs

Benchmark 2: python3 ./char_freq.py ~/projects/linux
  Time (mean ± σ):     32.407 s ±  0.309 s    [User: 31.972 s, System: 0.358 s]
  Range (min … max):   31.817 s … 32.823 s    10 runs

Summary
  ./charfreq-rs ~/projects/linux ran
  218.18 ± 7.08 times faster than python3 ./char_freq.py ~/projects/linux
```

TL;DR: The latest Rust version is ~218x faster than the original Python script.

## Improvements

- Testing
- Push performance further

## License

```
The MIT License (MIT)

Copyright (c) 2025-present PlumJam <git@plumj.am>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
