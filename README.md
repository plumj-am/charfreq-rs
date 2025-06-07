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

$ RUSTFLAGS="-C target-cpu=native" cargo build --profile=release
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
$ hyperfine --warmup=5 --runs=10 --shell=bash \
	'python3 ./char-freq/char_freq.py ./linux' \
	'./charfreq-rs/target/release/charfreq-rs -d ./linux' \
```
^ Compares latest to the original Python script.

### Results
```
Benchmark 1: 'python3 ./char_freq.py ./linux'
	Time (mean ± σ):     39.285 s ±  0.991 s
	Range (min … max):   38.220 s … 40.738 s

Benchmark 2: './charfreq-rs_1 -d ./linux'
	Time (mean ± σ):      1.376 s ±  0.022 s
	Range (min … max):    1.329 s …  1.402 s

Benchmark 3: './charfreq-rs_2 -d ./linux'
	Time (mean ± σ):      1.259 s ±  0.031 s
	Range (min … max):    1.211 s …  1.306 s

Benchmark 4: './charfreq-rs_3 -d ./linux'
	Time (mean ± σ):      1.224 s ±  0.026 s
	Range (min … max):    1.176 s …  1.263 s

Benchmark 5: './charfreq-rs_4 -d ./linux'
	Time (mean ± σ):     649.9 ms ±  18.5 ms
	Range (min … max):   615.8 ms … 675.7 ms
```
*NOTE: The hyperfine results have been edited solely to display them clearer.
The values have not been adjusted.*

#### Ranking
|rank|name                  |time (ms)                          |delta (ms)                                               |
|---:|:---------------------|----------------------------------:|--------------------------------------------------------:|
|1   |rust 0.4.0&nbsp;&nbsp;|  649.9&nbsp;&nbsp;                |±&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;18.5                |
|2   |rust 0.3.0&nbsp;&nbsp;| 1224&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|±&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;26&nbsp;&nbsp;&nbsp;|
|3   |rust 0.2.0&nbsp;&nbsp;| 1259&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|±&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;31&nbsp;&nbsp;&nbsp;|
|4   |rust 0.1.0&nbsp;&nbsp;| 1376&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|±&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;24&nbsp;&nbsp;&nbsp;|
|5   |python    &nbsp;&nbsp;|39285&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|±                 &nbsp;&nbsp;&nbsp;991&nbsp;&nbsp;&nbsp;|

Current version (0.4.0) is **~60.45x** faster than the original python script!

I'd appreciate if others could perform the same benchmarks and provide the
results along with their hardware information.

I'll happily add improvements to the rankings with credit. I'll need to test it
on my machine first, of course.

## Changelog for benchmarking

- 0.1.0: base
- 0.2.0: optimise build configuration
- 0.3.0: use mimalloc
- 0.4.0: efficient ascii handling

## Improvements

- Testing
- Push performance further
- Option for ignoring additional files
- Option for ignoring additional directories
- Simplify complex type `scanner.rs:143`
- Proper CSV support

## License

Copyright (c) James Plummer <jamesp2001@live.co.uk>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
