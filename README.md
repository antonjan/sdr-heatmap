# sdr-heatmap ![[](https://github.com/j2ghz/sdr-heatmap/actions?query=workflow%3ARust)](https://img.shields.io/github/workflow/status/j2ghz/sdr-heatmap/Rust) [![Codecov](https://img.shields.io/codecov/c/github/j2ghz/sdr-heatmap)](https://codecov.io/gh/j2ghz/sdr-heatmap) [![Coveralls github](https://img.shields.io/coveralls/github/j2ghz/sdr-heatmap)](https://coveralls.io/github/j2ghz/sdr-heatmap)

_I'm using this as an opportunity to learn Rust._


A tool to visualize files generated by [rtl_power](http://kmkeen.com/rtl-power/).
Inspired by [heatmap.py](https://github.com/keenerd/rtl-sdr-misc/blob/master/heatmap/heatmap.py).
It aims to provide better speed than heatmap.py, while keeping the same image format.
Another similar tool is [rtl-gopow](https://github.com/dhogborg/rtl-gopow), but it produces a different (IMO inferior) image than heatmap.py.

## Installation
<a href="https://repology.org/project/sdr-heatmap/versions"><img src="https://repology.org/badge/vertical-allrepos/sdr-heatmap.svg" alt="Packaging status" align="right"></a>

`cargo install sdr-heatmap` (from [crates.io](https://crates.io/crates/sdr-heatmap))

## Speed (WIP) (v0.1.2, singlethreaded)

|     Benchmark | sdr-heatmap |       heatmap.py |        rtl-gopow | Notes                                              |
| ------------: | ----------: | ---------------: | ---------------: | -------------------------------------------------- |
|         14 MB |      375 ms |          4558 ms |          1995 ms |                                                    |
| gzipped 44 MB |        14 s | failed after 3 m | gz not supported | Raspberry Pi 3B+                                   |
|         44 MB |        17 s |            801 s |            141 s | Raspberry Pi 3B+                                   |
|        829 MB |     19,37 s |         268,26 s |         141,34 s |                                                    |
|        829 MB |      611 MB |           427 MB |          5164 MB | _Peak memory, using `time -v`, single run, v0.1.3_ |

See raw results in [bench.md](bench.md)

## Roadmap
- [ ] Produce image like `heatmap.py`
  - [x] Signal values
  - [ ] Frequency header
  - [ ] Palettes
- [ ] Performance
  - [ ] Multithreading (currently singlethreaded)
  - [ ] Memory usage
- [ ] Formats
  - [x] PNG
  - [ ] WebP
  - [ ] Lossy?
  - [ ] Own, efficient, binary format
- [ ] Viewer
