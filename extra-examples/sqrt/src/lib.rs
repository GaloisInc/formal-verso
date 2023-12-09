#![no_std]
use core::cmp::min;
use soroban_sdk::{contractimpl, Env};

pub struct SqrtContract;

#[contractimpl]
impl SqrtContract {

    /// Compute sqrt, caching results in storage
    pub fn sqrt_cached(env: Env, x: u32) -> u32 {
        match env.storage().get(&x) {
            Some(y) => y.unwrap(),
            None => {
                let y = Self::sqrt(x);
                env.storage().set(&x, &y);
                y
            }
        }
    }

    /// An implementation of sqrt, ported from
    /// https://github.com/OpenZeppelin/openzeppelin-contracts/blob/552cffde563e83043a6c3a35012b626a25eba775/contracts/utils/math/Math.sol#L227
    pub fn sqrt(a: u32) -> u32 {
        if a == 0 {
            0
        } else {
            let mut result = 1 << (Self::log2(a) >> 1);
            result = (result + a / result) >> 1;
            result = (result + a / result) >> 1;
            result = (result + a / result) >> 1;
            result = (result + a / result) >> 1;
            min(result, a / result)
        }
    }

    /// Log base 2 ported from
    /// https://github.com/OpenZeppelin/openzeppelin-contracts/blob/552cffde563e83043a6c3a35012b626a25eba775/contracts/utils/math/Math.sol#L274
    pub fn log2(value: u32) -> u32 {
        let mut val = value;
        let mut result = 0;
        if val >> 16 > 0 {
            val >>= 16;
            result += 16;
        }
        if val >> 8 > 0 {
            val >>= 8;
            result += 8;
        }
        if val >> 4 > 0 {
            val >>= 4;
            result += 4;
        }
        if val >> 2 > 0 {
            val >>= 2;
            result += 2;
        }
        if val >> 1 > 0 {
            result += 1;
        }
        result
    }

}

mod test;
