#![cfg_attr(not(feature = "std"), no_std)]



use std::ops::Mul;

use liquid_primitives::types::{u256, i256};

pub fn shl(o:u32,v:&u256)->u256{
    // v.mul(u256(u256::from(2).pow(0)))
    let result = v.0.clone()<<o;
    u256(result)
}

pub fn gt(o:&u256,v:&u256)->u256{
    if o.ge(v){
        "1".parse().unwrap()
    }else{
        "0".parse().unwrap()
    }
}

pub fn or(o:&u256,v:&u256)->u256{
    let result = o.0.clone()|v.0.clone();
    u256(result)
}

pub fn or_i(o:&i256,v:&u256)->i256{
    let result = o.0.clone()|v.to_int256().unwrap().0.clone();
    i256(result)
}

pub fn shr(o:u32,v:&u256)->u256{
    let result = v.0.clone()>>o;
    u256(result)

}

pub fn mul(o:&u256,v:&u256)->u256{
    o.clone().mul(v.clone())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use liquid_primitives::types::u256;
    use primitives::U256;



    #[test]
    fn it_work(){
        let a = U256::from_dec_str("1235546464564589484565647822").unwrap();
        let mut little_endian = [0u8;32];
        a.to_little_endian(&mut little_endian);
        let b=u256::from_le_bytes(&little_endian);
        // let result:u256 = "0x10".parse().unwrap();
        println!("b is:{:?}",b);
    }
}