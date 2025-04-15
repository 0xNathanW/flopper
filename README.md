# Flopper

A poker equity solver using the fastest hand evualation algorithm created by Ray Wotton and others on teh 2+2 poker forum.

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

```