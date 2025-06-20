Removed from the README.md because I don't think the tests were optimal. System
wasn't quiet, CPU OC was unstable and warmup/runs might not have been enough.

New results in the README compare the latest Rust version to the original Python
script since that's what's most interesting. Additionally, the latest results
are on a fresh system with a stable CPU OC. I'd like to do it again with better
storage at a later date.

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

Benchmark 6: './charfreq-rs_5 -d ./linux'
	Time (mean ± σ):     630.2 ms ±  30.8 ms
	Range (min … max):   591.7 ms … 666.5 ms
```

#### Ranking
|rank|name                  |time (ms)                          |delta (ms)                                               |
|---:|:---------------------|----------------------------------:|--------------------------------------------------------:|
|1   |rust 0.5.0&nbsp;&nbsp;|  630.2&nbsp;&nbsp;                |±&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;30.8                |
|1   |rust 0.4.0&nbsp;&nbsp;|  649.9&nbsp;&nbsp;                |±&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;18.5                |
|2   |rust 0.3.0&nbsp;&nbsp;| 1224&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|±&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;26&nbsp;&nbsp;&nbsp;|
|3   |rust 0.2.0&nbsp;&nbsp;| 1259&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|±&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;31&nbsp;&nbsp;&nbsp;|
|4   |rust 0.1.0&nbsp;&nbsp;| 1376&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|±&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;24&nbsp;&nbsp;&nbsp;|
|5   |python    &nbsp;&nbsp;|39285&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|±                 &nbsp;&nbsp;&nbsp;991&nbsp;&nbsp;&nbsp;|

- (0.5.0) is **~62.34x** faster than the original python script.
- (0.4.0) is **~60.45x** faster than the original python script.
- (0.3.0) is **~32.10x** faster than the original python script.
- (0.2.0) is **~31.20x** faster than the original python script.
- (0.1.0) is **~28.55x** faster than the original python script.
