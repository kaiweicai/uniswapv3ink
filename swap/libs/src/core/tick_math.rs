#![cfg_attr(not(feature = "std"), no_std)]

use std::ops::{Sub, Div, Mul, Add};

use liquid_primitives::types::{u256, i256};
use primitives::{Int24, Uint160, U160, U256, int256, uint256};
use sp_core::crypto::UncheckedInto;

use crate::{assembly::{liquid_cal::{cal_ratio, cal_log}, liquid_assembly::{gt, or}}};


/// @dev The minimum tick that may be passed to #getSqrtRatioAtTick computed from log base 1.0001 of 2**-128
pub const MIN_TICK:Int24 = -887272;
    /// @dev The maximum tick that may be passed to #getSqrtRatioAtTick computed from log base 1.0001 of 2**128
pub const MAX_TICK:Int24 = -MIN_TICK;

/// @dev The minimum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MIN_TICK)
pub const MIN_SQRT_RATIO:&str = "4295128739";//4295128739;
/// @dev The maximum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MAX_TICK)
pub const MAX_SQRT_RATIO:&str = "1461446703485210103287273052203988822378723970342";//1461446703485210103287273052203988822378723970342;

/// parameter sqrt_price_x96 is U160
pub fn get_tick_at_sqrt_ratio(sqrt_price_x96:&u256)->Int24{
    // 0
    // second inequality must be < because the price can never reach the price at the max tick
    // require(sqrtPriceX96 >= MIN_SQRT_RATIO && sqrtPriceX96 < MAX_SQRT_RATIO, 'R');
    // uint256 ratio = uint256(sqrtPriceX96) << 32;
    assert!(sqrt_price_x96.ge(&MIN_SQRT_RATIO.parse().unwrap())&&sqrt_price_x96.gt(&MAX_SQRT_RATIO.parse().unwrap()),"R");
    let ratio:u256 = sqrt_price_x96.clone().div(u256::from(32u32));
    
    // uint256 r = ratio;
    // uint256 msb = 0;
    let r = ratio.clone();
    let msb = u256::default();

    // assembly {
    //     let f := shl(7, gt(r, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,7u32,"0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    // assembly {
    //     let f := shl(6, gt(r, 0xFFFFFFFFFFFFFFFF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,6u32,"0xFFFFFFFFFFFFFFFF");
    // assembly {
    //     let f := shl(5, gt(r, 0xFFFFFFFF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,5u32,"0xFFFFFFFF");
    // assembly {
    //     let f := shl(4, gt(r, 0xFFFF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,4u32,"0xFFFF");
    // assembly {
    //     let f := shl(3, gt(r, 0xFF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,3u32,"0xFF");
    // assembly {
    //     let f := shl(2, gt(r, 0xF))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,r) = cal_ratio(&msb,&r,2u32,"0xF");
    // assembly {
    //     let f := shl(1, gt(r, 0x3))
    //     msb := or(msb, f)
    //     r := shr(f, r)
    // }
    let (msb,mut r) = cal_ratio(&msb,&r,1u32,"0x3");
    // assembly {
    //     let f := gt(r, 0x1)
    //     msb := or(msb, f)
    // }
    let f = gt(&r,&"0x1".parse().unwrap());
    let msb = or(&msb,&f);

    // if (msb >= 128) r = ratio >> (msb - 127);
    // else r = ratio << (127 - msb);
    if msb.ge(&u256::from(128u32)){
        r = u256(ratio.0 >> 127u32);
    }else{
        r = u256(ratio.0 << (127u32-msb.to_u32_digits()[0]));
    }

    // int256 log_2 = (int256(msb) - 128) << 64;
    let log_2:i256 = i256(msb.to_int256().unwrap().sub(i256::from(128u32)).0<<64u32);

    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(63, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r, &log_2, 63);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(62, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r, &log_2, 62);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(61, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r, &log_2, 61);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(60, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r, &log_2, 60);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(59, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r, &log_2, 59);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(58, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r, &log_2, 58);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(57, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r, &log_2, 57);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(56, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r, &log_2, 56);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(55, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r, &log_2, 55);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(54, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r, &log_2, 54);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(53, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r, &log_2, 53);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(52, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r, &log_2, 52);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(51, f))
    //     r := shr(f, r)
    // }
    let (log_2,r) = cal_log(&r, &log_2, 51);
    // assembly {
    //     r := shr(127, mul(r, r))
    //     let f := shr(128, r)
    //     log_2 := or(log_2, shl(50, f))
    // }
    let (log_2,r) = cal_log(&r, &log_2, 50);
    // int256 log_sqrt10001 = log_2 * 255738958999603826347141; // 128.128 number
    let log_sqrt10001:i256 = log_2.mul("255738958999603826347141".parse().unwrap());
    // int24 tickLow = int24((log_sqrt10001 - 3402992956809132418596140100660247210) >> 128);
    // int24 tickHi = int24((log_sqrt10001 + 291339464771989622907027621153398088495) >> 128);
    let tick_low:Int24 = i32::from_str_radix((log_sqrt10001.clone().sub("3402992956809132418596140100660247210".parse().unwrap()).0>>128u32).to_string().as_str(),10).unwrap();
    let tick_hi:Int24 = i32::from_str_radix((log_sqrt10001.clone().add("291339464771989622907027621153398088495".parse().unwrap()).0>>128u32).to_string().as_str(),10).unwrap();

    // tick = tickLow == tickHi ? tickLow : getSqrtRatioAtTick(tickHi) <= sqrtPriceX96 ? tickHi : tickLow;
    let tick:Int24;
    tick = if tick_low == tick_hi {
        tick_low
    }else if get_sqrt_ratio_at_tick(&tick_hi).ge(sqrt_price_x96){
        tick_hi
    }else{
        tick_low
    };
    tick
}

/// @notice Calculates sqrt(1.0001^tick) * 2^96
    /// @dev Throws if |tick| > max tick
    /// @param tick The input tick for the above formula
    /// @return sqrtPriceX96 A Fixed point Q64.96 number representing the sqrt of the ratio of the two assets (token1/token0)
    /// at the given tick
pub fn get_sqrt_ratio_at_tick(tick:&Int24) -> u256{
        // uint256 absTick = tick < 0 ? uint256(-int256(tick)) : uint256(int256(tick));
        // require(absTick <= uint256(MAX_TICK), 'T');
        let abs_tick = tick.abs();
        assert!(abs_tick.le(&MAX_TICK),"T");
        
        // uint256 ratio = absTick & 0x1 != 0 ? 0xfffcb933bd6fad37aa2d162d1a594001 : 0x100000000000000000000000000000000;
        let mut ratio:u256;
        if abs_tick & 1i32 !=0 {
            ratio = "0xfffcb933bd6fad37aa2d162d1a594001".parse().unwrap();
        }else {
            ratio = "0x100000000000000000000000000000000".parse().unwrap();
        }
        // if (absTick & 0x2 != 0) ratio = (ratio * 0xfff97272373d413259a46990580e213a) >> 128;
        if abs_tick & 2i32.pow(1) != 0{
            ratio = u256(ratio.mul("0xfff97272373d413259a46990580e213a".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x4 != 0) ratio = (ratio * 0xfff2e50f5f656932ef12357cf3c7fdcc) >> 128;
        if abs_tick & 2i32.pow(2) != 0{
            ratio = u256(ratio.mul("0xfff2e50f5f656932ef12357cf3c7fdcc".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x8 != 0) ratio = (ratio * 0xffe5caca7e10e4e61c3624eaa0941cd0) >> 128;
        if abs_tick & 2i32.pow(3) != 0{
            ratio = u256(ratio.mul("0xffe5caca7e10e4e61c3624eaa0941cd0".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x10 != 0) ratio = (ratio * 0xffcb9843d60f6159c9db58835c926644) >> 128;
        if abs_tick & 2i32.pow(4) != 0{
            ratio = u256(ratio.mul("0xffcb9843d60f6159c9db58835c926644".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x20 != 0) ratio = (ratio * 0xff973b41fa98c081472e6896dfb254c0) >> 128;
        if abs_tick & 2i32.pow(5) != 0{
            ratio = u256(ratio.mul("0xff973b41fa98c081472e6896dfb254c0".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x40 != 0) ratio = (ratio * 0xff2ea16466c96a3843ec78b326b52861) >> 128;
        if abs_tick & 2i32.pow(6) != 0{
            ratio = u256(ratio.mul("0xff2ea16466c96a3843ec78b326b52861".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x80 != 0) ratio = (ratio * 0xfe5dee046a99a2a811c461f1969c3053) >> 128;
        if abs_tick & 2i32.pow(7) != 0{
            ratio = u256(ratio.mul("0xfe5dee046a99a2a811c461f1969c3053".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x100 != 0) ratio = (ratio * 0xfcbe86c7900a88aedcffc83b479aa3a4) >> 128;
        if abs_tick & 2i32.pow(8) != 0{
            ratio = u256(ratio.mul("0xfcbe86c7900a88aedcffc83b479aa3a4".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x200 != 0) ratio = (ratio * 0xf987a7253ac413176f2b074cf7815e54) >> 128;
        if abs_tick & 2i32.pow(9) != 0{
            ratio = u256(ratio.mul("0xf987a7253ac413176f2b074cf7815e54".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x400 != 0) ratio = (ratio * 0xf3392b0822b70005940c7a398e4b70f3) >> 128;
        if abs_tick & 2i32.pow(10) != 0{
            ratio = u256(ratio.mul("0xf3392b0822b70005940c7a398e4b70f3".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x800 != 0) ratio = (ratio * 0xe7159475a2c29b7443b29c7fa6e889d9) >> 128;
        if abs_tick & 2i32.pow(11) != 0{
            ratio = u256(ratio.mul("0xe7159475a2c29b7443b29c7fa6e889d9".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x1000 != 0) ratio = (ratio * 0xd097f3bdfd2022b8845ad8f792aa5825) >> 128;
        if abs_tick & 2i32.pow(12) != 0{
            ratio = u256(ratio.mul("0xd097f3bdfd2022b8845ad8f792aa5825".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x2000 != 0) ratio = (ratio * 0xa9f746462d870fdf8a65dc1f90e061e5) >> 128;
        if abs_tick & 2i32.pow(13) != 0{
            ratio = u256(ratio.mul("0xa9f746462d870fdf8a65dc1f90e061e5".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x4000 != 0) ratio = (ratio * 0x70d869a156d2a1b890bb3df62baf32f7) >> 128;
        if abs_tick & 2i32.pow(14) != 0{
            ratio = u256(ratio.mul("0x70d869a156d2a1b890bb3df62baf32f7".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x8000 != 0) ratio = (ratio * 0x31be135f97d08fd981231505542fcfa6) >> 128;
        if abs_tick & 2i32.pow(15) != 0{
            ratio = u256(ratio.mul("0x31be135f97d08fd981231505542fcfa6".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x10000 != 0) ratio = (ratio * 0x9aa508b5b7a84e1c677de54f3e99bc9) >> 128;
        if abs_tick & 2i32.pow(16) != 0{
            ratio = u256(ratio.mul("0x9aa508b5b7a84e1c677de54f3e99bc9".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x20000 != 0) ratio = (ratio * 0x5d6af8dedb81196699c329225ee604) >> 128;
        if abs_tick & 2i32.pow(17) != 0{
            ratio = u256(ratio.mul("0x5d6af8dedb81196699c329225ee604".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x40000 != 0) ratio = (ratio * 0x2216e584f5fa1ea926041bedfe98) >> 128;
        if abs_tick & 2i32.pow(18) != 0{
            ratio = u256(ratio.mul("0x2216e584f5fa1ea926041bedfe98".parse().unwrap()).0 >> 128u32);
        }
        // if (absTick & 0x80000 != 0) ratio = (ratio * 0x48a170391f7dc42444e8fa2) >> 128;
        if abs_tick & 2i32.pow(19) != 0{
            ratio = u256(ratio.mul("0x48a170391f7dc42444e8fa2".parse().unwrap()).0 >> 128u32);
        }

        // if (tick > 0) ratio = type(uint256).max / ratio;
        if tick.gt(&0i32) {
            ratio = u256::from_le_bytes(&[255u8;64]).div(ratio);
        }

        // // this divides by 1<<32 rounding up to go from a Q128.128 to a Q128.96.
        // // we then downcast because we know the result always fits within 160 bits due to our tick input constraint
        // // we round up in the division so getTickAtSqrtRatio of the output price is always consistent
        // sqrtPriceX96 = uint160((ratio >> 32) + (ratio % (1 << 32) == 0 ? 0 : 1));
        //此处需要根据余数判断是否需要增加一个点的手续费,先简化为直接加一个点.TODO 后期做修改.
        let sqrt_price_x96:u256 = u256(ratio.0>>32u32).add(u256::from(1u32));
        sqrt_price_x96
    }