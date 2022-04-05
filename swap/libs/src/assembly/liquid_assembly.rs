#![cfg_attr(not(feature = "std"), no_std)]



use std::ops::Mul;

use liquid_primitives::types::u256;

pub fn shl(o:u128,v:&u256)->u256{
    // v.mul(u256(u256::from(2).pow(0)))
    let result = v.0.clone()<<o;
    u256(result)
}

pub fn gt(o:&u256,v:&u256)->u32{
    if o.ge(v){
        1u32
    }else{
        0u32
    }
}

pub fn or(o:&u256,v:&u256)->u256{
    let result = o.0.clone()|v.0.clone();
    u256(result)
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
    use liquid_primitives::types::u256;



    #[test]
    fn it_work(){
        let result:u256 = "1461446703485210103287273052203988822378723970342".parse().unwrap();
        println!("result is:{:?}",result);
    }
}