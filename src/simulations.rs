// Importing the U256 type from the ethers crate
use ethers::types::U256;

// Defining a struct to represent a Tick
pub struct Tick {
    pub liquidity_gross: u128,
    pub liquidity_net: i128,
    pub fee_growth_outside_0_x_128: U256,
    pub fee_growth_outside_1_x_128: U256,
    pub tick_cumulative_outside: U256,
    pub seconds_per_liquidity_outside_x_128: U256,
    pub seconds_outside: u32,
    pub initialized: bool,
}

// Function to calculate the fee growth inside a given range
#[allow(unused_assignments)]
pub fn get_fee_growth_inside(
    tick_lower: i32,                 // Lower bound of the range
    tick_upper: i32,                 // Upper bound of the range
    tick_current: i32,               // Current tick
    fee_growth_global_0_x_128: U256, // Global fee growth for token 0
    fee_growth_global_1_x_128: U256, // Global fee growth for token 1
    lower: &Tick,                    // Tick at the lower bound
    upper: &Tick,                    // Tick at the upper bound
) -> (U256, U256) {
    // Returns fee growth inside for token 0 and token 1
    // Initialize fee growth variables
    let (
        mut fee_growth_below_0_x_128,
        mut fee_growth_below_1_x_128,
        mut fee_growth_above_0_x_128,
        mut fee_growth_above_1_x_128,
    ) = (U256::zero(), U256::zero(), U256::zero(), U256::zero());

    // Calculate fee growth below current tick
    if tick_current >= tick_lower {
        fee_growth_below_0_x_128 = lower.fee_growth_outside_0_x_128;
        fee_growth_below_1_x_128 = lower.fee_growth_outside_1_x_128;
    } else {
        fee_growth_below_0_x_128 = fee_growth_global_0_x_128 - lower.fee_growth_outside_0_x_128;
        fee_growth_below_1_x_128 = fee_growth_global_1_x_128 - lower.fee_growth_outside_1_x_128;
    }

    // Calculate fee growth above current tick
    if tick_current < tick_upper {
        fee_growth_above_0_x_128 = upper.fee_growth_outside_0_x_128;
        fee_growth_above_1_x_128 = upper.fee_growth_outside_1_x_128;
    } else {
        fee_growth_above_0_x_128 = fee_growth_global_0_x_128 - upper.fee_growth_outside_0_x_128;
        fee_growth_above_1_x_128 = fee_growth_global_1_x_128 - upper.fee_growth_outside_1_x_128;
    }

    // Calculate fee growth inside the range
    let fee_growth_inside_0_x_128 =
        if fee_growth_global_0_x_128 > fee_growth_below_0_x_128 + fee_growth_above_0_x_128 {
            fee_growth_global_0_x_128 - fee_growth_below_0_x_128 - fee_growth_above_0_x_128
        } else {
            U256::zero()
        };

    let fee_growth_inside_1_x_128 =
        if fee_growth_global_1_x_128 > fee_growth_below_1_x_128 + fee_growth_above_1_x_128 {
            fee_growth_global_1_x_128 - fee_growth_below_1_x_128 - fee_growth_above_1_x_128
        } else {
            U256::zero()
        };

    // Return fee growth inside for token 0 and token 1
    (fee_growth_inside_0_x_128, fee_growth_inside_1_x_128)
}

pub fn to_real(num: U256) -> f64 {
    let divisor = U256::exp10(38);
    let quotient = num / &divisor;
    let remainder = num % &divisor;

    // Convert quotient and remainder to f64
    let quotient_f64 = quotient.low_u64() as f64;
    let divisor_f64 = divisor.low_u64() as f64;
    let remainder_f64 = remainder.low_u64() as f64 / divisor_f64;

    quotient_f64 + remainder_f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::types::U256;

    #[test]
    fn test_get_fee_growth_inside() {
        let lower = Tick {
            liquidity_gross: 1000,
            liquidity_net: 500,
            fee_growth_outside_0_x_128: U256::from(1000),
            fee_growth_outside_1_x_128: U256::from(2000),
            tick_cumulative_outside: U256::zero(),
            seconds_per_liquidity_outside_x_128: U256::zero(),
            seconds_outside: 0,
            initialized: true,
        };

        let upper = Tick {
            liquidity_gross: 2000,
            liquidity_net: 1000,
            fee_growth_outside_0_x_128: U256::from(3000),
            fee_growth_outside_1_x_128: U256::from(4000),
            tick_cumulative_outside: U256::zero(),
            seconds_per_liquidity_outside_x_128: U256::zero(),
            seconds_outside: 0,
            initialized: true,
        };

        let (fee_growth_inside_0_x_128, fee_growth_inside_1_x_128) = get_fee_growth_inside(
            -10,
            10,
            0,
            U256::from(5000),
            U256::from(6000),
            &lower,
            &upper,
        );

        assert_eq!(fee_growth_inside_0_x_128, U256::from(1000));
        assert_eq!(fee_growth_inside_1_x_128, U256::from(0));
    }
}
