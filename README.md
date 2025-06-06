# charfreq-rs 🦀

Count the occurrences of characters in a codebase or any directory.

A Rust rewrite of [https://github.com/jamesukiyo/char-freq](https://github.com/jamesukiyo/char-freq).

The original Python implementation was created to determine the symbols I use
most when writing code so I could optimise the layout on my split keyboard.

My first actual project written in Rust outside of learning/exercises so this
was mostly for practice.

If improvements can be made, please open a PR or issue! I suspect this is far
from perfect and I plan to make it better myself.

## Usage:

```
$ git clone https://github.com/jamesukiyo/charfreq-rs.git

$ cd charfreq-rs

$ RUST_FLAGS="-C target-cpu=native" cargo build --profile=release
```
Binary can be found at `./target/release/charfreq-rs`.
```
Usage: charfreq-rs [OPTIONS]

Options:
  -d, --dir <REPO_PATH>  Path to the repository [default: ""]
  -t, --top <TOP>        Number of top characters to display [default: 50]
  -s, --show-spaces      Include spaces and whitespace characters in the output
  -e, --exclude-letters  Exclude all letters (A-Z, a-z) from the output
      --save-csv         [MAY NOT WORK] Save results as CSV in the current working directory
  -h, --help             Print help
```
Example:
```
$ ./charfreq-rs -d ~/projects/charfreq-rs --top 5 --exclude-letters
```
Will show the top 5 non-alphabetic characters in a codebase.

*Note: Many filetypes (e.g. `.exe`, `.mp3`) and directories
(e.g.`node_modules/`, `.idea/`) are ignored by default. At this time, there are
no CLI options to adjust this, they must be added manually in `src/scanner.rs`.
A full list of ignored filetypes and directories can be found there too.*

## Benchmarks

Comparison between both implementations (Rust iterations vs Python).

### Test

**Tool**: **[hyperfine](https://github.com/sharkdp/hyperfine)**

**Tested on**:
- Linux kernel source tree: **[torvalds/linux](https://github.com/torvalds/linux)**
- `~82_333` files
- `~1_508_915_498` characters

**Hardware**:
- `i5-13600KF @5.3GHz (OC)`,
- `2x16GB DDR5 G.Skill Z5 Trident @7000MT/s (OC)`,
- `WD 250GB SATA SSD` (generic, cheap model)

```sh
$ hyperfine --warmup 3 --runs 10 --shell=bash \
	'python3 ./char-freq/char_freq.py ./linux' \
	'./charfreq-rs/target/release/charfreq-rs_1 -d ./linux' \
	'./charfreq-rs/target/release/charfreq-rs_2 -d ./linux' \
	'./charfreq-rs/target/release/charfreq-rs_3 -d ./linux' \
```
You can also just run them individually.

### Results
```
Benchmark 1: char_freq.py ./linux				// Python
	Time (mean ± σ):     39.356 s ±  1.383 s

Benchmark 2: charfreq-rs_1 -d ./linux			// Rust base
	Time (mean ± σ):      1.383 s ±  0.024 s

Benchmark 3: charfreq-rs_2 -d ./linux			// Build optimisations
	Time (mean ± σ):      1.258 s ±  0.024 s

Benchmark 4: charfreq-rs_3 -d ./linux			// Use mimalloc
	Time (mean ± σ):      1.220 s ±  0.026 s
```

*NOTE: The hyperfine results have been edited solely to display them clearer.
The values have not been adjusted.*

#### Ranking
1. rust 0.3.0 |  1.220s ± 0.026s
2. rust 0.2.0 |  1.258s ± 0.024s
3. rust 0.1.0 |  1.383s ± 0.024s
4. python     | 39.356s ± 1.383s

I'd appreciate if others could perform the same benchmarks and provide the
results along with their hardware information!

I'll happily add improvements to the rankings with credit. I'll need to test it
on my machine first, of course.

## Changelog for benchmarking

- 0.1.0: base
- 0.2.0: optimise build configuration
- 0.3.0: use mimalloc

## Improvements

- Testing
- Push performance further
- Option for ignoring additional files
- Option for ignoring additional directories

## License

Copyright (c) James Plummer <jamesp2001@live.co.uk>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
