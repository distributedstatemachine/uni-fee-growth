pub mod simulations;
pub mod data_adapters {
    pub mod the_graph;
}
// Import the necessary crates
use clap::Parser;
use ethers::types::U256;
use simulations::{get_fee_growth_inside, Tick};
use data_adapters::the_graph::{get_pool_data,get_subgraph_url};

// // Define your command-line arguments
// #[derive(Parser)]
// struct Opts {
//     /// The lower bound of the range
//     #[clap( long)]
//     tick_lower: i32,

//     /// The upper bound of the range
//     #[clap(long)]
//     tick_upper: i32,

//     /// The current tick
//     #[clap( long)]
//     tick_current: i32,

//     /// The global fee growth for token 0
//     #[clap(long)]
//     fee_growth_global_0_x_128: u64,

//     /// The global fee growth for token 1
//     #[clap(long)]
//     fee_growth_global_1_x_128: u64,
// }

// fn main() {
//     let opts: Opts = Opts::parse();

//     // Initialize variables
//     let tick_lower = opts.tick_lower;
//     let tick_upper = opts.tick_upper;
//     let tick_current = opts.tick_current;
//     let fee_growth_global_0_x_128 = U256::from(opts.fee_growth_global_0_x_128);
//     let fee_growth_global_1_x_128 = U256::from(opts.fee_growth_global_1_x_128);

//     let lower = Tick {
//         liquidity_gross: 1000,
//         liquidity_net: 500,
//         fee_growth_outside_0_x_128: U256::from(1000),
//         fee_growth_outside_1_x_128: U256::from(2000),
//         tick_cumulative_outside: U256::zero(),
//         seconds_per_liquidity_outside_x_128: U256::zero(),
//         seconds_outside: 0,
//         initialized: true,
//     };
//     let upper = Tick {
//         liquidity_gross: 2000,
//         liquidity_net: 1000,
//         fee_growth_outside_0_x_128: U256::from(3000),
//         fee_growth_outside_1_x_128: U256::from(4000),
//         tick_cumulative_outside: U256::zero(),
//         seconds_per_liquidity_outside_x_128: U256::zero(),
//         seconds_outside: 0,
//         initialized: true,
//     };
//     // Call the function and print the results
//     let (fee_growth_inside_0_x_128, fee_growth_inside_1_x_128) = get_fee_growth_inside(
//         tick_lower,
//         tick_upper,
//         tick_current,
//         fee_growth_global_0_x_128,
//         fee_growth_global_1_x_128,
//         &lower,
//         &upper,
//     );

//     println!("fee_growth_inside_0_x_128: {}", fee_growth_inside_0_x_128);
//     println!("fee_growth_inside_1_x_128: {}", fee_growth_inside_1_x_128);
// }

#[derive(clap::Parser)]
struct Opts {
    #[clap(short, long)]
    chain: String,

    /// The Uniswap V3 token pair address
    #[clap(short, long)]
    pair: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let chain = &opts.chain;
    let pair = &opts.pair;

    let subgraph_url = get_subgraph_url(chain);
    let pool_data = get_pool_data(subgraph_url, pair);

    match pool_data {
        Some(data) => {
            // Process the data as needed
            println!("Pool data: {:?}", data);
        },
        None => {
            println!("No data found for pool {}", pair);
        }
    }
}