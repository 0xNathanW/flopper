# Flopper

A poker equity solver using the fastest hand evualation algorithm created by Ray Wotton and others on the 2+2 poker forum.

## Usage

First we have to generate the lookup table, do this by running the `generate_lookup` binary.  This only has to be done once.

```
Usage: generate_lookup --path <PATH>

Options:
  -p, --path <PATH>  Path to save the lookup table
  -h, --help         Print help
  ```
Running with release mode this won't take long; 5 seconds on an AMD Ryzen 7 3700X.
```
cargo run --bin generate_lookup -r -- --path ./data.bin
[00:00:00] Sub hands enumeration completed
[00:00:05] [========================================] 612977/612977 (100%)
Saving lookup table to ./data.bin...
```
Then you can run the `equity` binary.
```
Usage: equity [OPTIONS] --lookup <LOOKUP> [RANGES]...

Arguments:
  [RANGES]...  String represention of ranges to compare. Eg. '22-77' 'A2s+, KQs'

Options:
  -b, --board <BOARD>            Board cards (0-5). Eg. '8d Tc 2h', empty for no board
  -l, --lookup <LOOKUP>          Path to lookup table
  -m, --monte-carlo              Use Monte Carlo simulation instead of enumeration
  -i, --iterations <ITERATIONS>  Number of iterations for Monte Carlo simulation (default: run until SIGINT)
  -h, --help                     Print help
  -V, --version                  Print version
```
```
cargo run --bin equity -r -- "77+, A9s+, KTs+, AJo+" "44+, A2s+, K9s+, Q9s+, J9s+, T9s, 98s, 87s, 76s, ATo+, KJo+" --lookup ./data/lookup_table.bin --board "Tc 3s 2d"
Enumerating 1,176 runouts
[00:00:00] [########################################] 1,176/1,176
+-------------------------------------------------------------+--------+--------+-------+
| Range                                                       | Equity | Win %  | Tie % |
+-------------------------------------------------------------+--------+--------+-------+
| 77+, A9s+, KTs+, AJo+                                       | 57.58% | 56.77% | 1.61% |
+-------------------------------------------------------------+--------+--------+-------+
| 44+, A2s+, K9s+, Q9s+, J9s+, T9s, 98s, 87s, 76s, ATo+, KJo+ | 40.81% | 40.01% | 1.61% |
+-------------------------------------------------------------+--------+--------+-------+
```
## Tests/Benchmarks
For testing and benchmarking the lookup is loaded from the environment variable `LOOKUP_PATH` with `data/lookup_table.bin` as default.
