#![cfg_attr(not(feature = "std"), no_std)]


use liquid_primitives::types::{u256, i256};

use super::liquid_assembly::{shl, gt, or, shr, mul, or_i};

/// str is "0xff"
/// o is 7
pub fn cal_ratio(msb:&u256,r:&u256,o:u32,v:&'static str)->(u256,u256){
    // let f := shl(7, gt(r, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF))
    //         msb := or(msb, f)
    //         r := shr(f, r)
    let v:u256 = v.parse().unwrap();
    let f = shl(o,&gt(r,&v));
    let msb = or(msb,&f);
    let r = shr(f.to_u32_digits()[0],r); 
    (msb,r)
}

///w is 63
pub fn cal_log(r:&u256,log_2:&i256,w:u32)->(i256,u256){
    // r := shr(127, mul(r, r))
    //         let f := shr(128, r)
    //         log_2 := or(log_2, shl(63, f))
    //         r := shr(f, r)
    let r = shr("127".parse().unwrap(),&mul(r,r));
    let f = shr("128".parse().unwrap(),&r);
    let log_2 = or_i(&log_2,&shl(w, &f));
    let r = shr(f.to_u32_digits()[0],&r);
    (log_2,r)
}


#[cfg(test)]
mod tests {
    use primitives::U256;

    use super::cal_ratio;

    #[test]
    fn it_works() {
        let s = U256::from("FF");
        println!("test result is:{:?}",s.to_string());
        println!("test hex result is:{:?}",hex::decode("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"));
    }

    #[test]
    fn test_result() {
        let mut r = "12312312312312312".parse().unwrap();
        let mut msb = "0".parse().unwrap();
        let o =2u32;
        let v ="0xF";
        (msb,r) = cal_ratio(&r,&msb,o,v);
        println!("test msb is:{:?}",msb.to_string());
        println!("test r is:{:?}",r.to_string());
    }

    #[test]
    fn it_work(){
        let result = U256::from_dec_str("1461446703485210103287273052203988822378723970342");
        println!("result is:{:?}",result);
    }
}