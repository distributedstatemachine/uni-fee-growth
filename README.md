# Uniswap v3 Fee Growth

This is a script to calculate fee growth in a given tick space. 

## Prerequisites

- Ensure [Rust](https://www.rust-lang.org/tools/install) is installed. 

## Running 

- From the root of the directory , run

```shell
cargo run -- --tick-lower 10 --tick-upper 20 --tick-current 15 --fee-growth-global-0-x-128 5000 --fee-growth-global-1-x-1
28 6000
```


## TODO

- [ ] Handle solidity coversions
- [ ] Connect to real data sources
- [ ] Calculate Reserves
- [ ] Visualise
- [ ] Render to dataframes using polars
