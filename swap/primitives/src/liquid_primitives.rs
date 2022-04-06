#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::AccountId;
// SPDX-License-Identifier: GPL-2.0-or-later

#[cfg(feature = "std")]
use ink_metadata::layout::{StructLayout, Layout, FieldLayout};
use ink_storage::{
    traits::{PackedLayout, SpreadLayout, StorageLayout, SpreadAllocate,ExtKeyPtr},
};
use liquid_primitives::types::u256;
// use primitive_types::u256;
use scale::{Decode, Encode};


#[cfg(feature = "std")]
use scale_info::{TypeInfo, Type};
pub type Address = AccountId;
pub type Uint24 = u32;
pub type Uint16 = u16;
pub type Int24 = i32;
pub type Uint8 = u8;
pub type Uint160 = WrapperU256;
pub type Uint256 = WrapperU256;
pub type U160 = u256;
pub type U256 = u256;
pub type I56 = i64;

pub const ADDRESS0:[u8;32] = [0u8;32];

#[derive(Default,Debug,Clone, PartialEq, Eq,Encode, Decode)]
// #[cfg_attr(feature = "std", derive(TypeInfo))]
pub struct WrapperU256 {
    pub value: u256,
}

impl WrapperU256{
    pub fn new(v:&[u8])->Self{
        WrapperU256{
            value:u256::from_be_bytes(v),
        }
    }
}

impl AsRef<u256> for WrapperU256 {
    #[inline]
    fn as_ref(&self) ->  &u256{
      &self.value
    }
}

#[cfg(feature = "std")]
impl TypeInfo for WrapperU256
{
    type Identity = [u64];

    fn type_info() -> Type {
        Self::Identity::type_info()
    }
}

// #[cfg(feature = "std")]
// impl WrapperTypeEncode for WrapperU256{
    
// }

// #[cfg(feature = "std")]
// impl Deref for WrapperU256{
//     type Target=[u64];

//     fn deref(&self) -> &Self::Target {
//         self.value.as_ref()
//     }
// }

impl SpreadLayout for WrapperU256 {
    const FOOTPRINT: u64 = 4;
    const REQUIRES_DEEP_CLEAN_UP: bool = true;
    fn pull_spread(ptr: &mut ink_primitives::KeyPtr) -> Self {
        let slice: [u8; 64] = SpreadLayout::pull_spread(ptr);
        Self { value: u256::from_be_bytes(&slice) }
    }

    fn push_spread(&self, ptr: &mut ink_primitives::KeyPtr) {
        SpreadLayout::push_spread(&self.value.0.data, ptr);
    }

    fn clear_spread(&self, ptr: &mut ink_primitives::KeyPtr) {
        SpreadLayout::clear_spread(&self.value.0, ptr);
    }
}

impl PackedLayout for WrapperU256 {
    fn pull_packed(&mut self, at: &ink_primitives::Key) {
        self.value.0.pull_packed(at);
    }

    fn push_packed(&self, at: &ink_primitives::Key) {
        self.value.0.push_packed(at);
    }

    fn clear_packed(&self, at: &ink_primitives::Key) {
        self.value.0.clear_packed(at);
    }
}

impl SpreadAllocate for WrapperU256{
    fn allocate_spread(ptr: &mut ink_primitives::KeyPtr) -> Self {
        ptr.next_for::<WrapperU256>();
        // Id::U8(0)
        WrapperU256::new([0u64;4])
    }
}

#[cfg(feature = "std")]
impl StorageLayout for WrapperU256 {
    fn layout(key_ptr: &mut ink_primitives::KeyPtr) -> Layout {
        Layout::Struct(StructLayout::new([
            FieldLayout::new(
                "len",
                <[u32; 4] as StorageLayout>::layout(key_ptr),
            ),
            FieldLayout::new("elems", <[u32; 6] as StorageLayout>::layout(key_ptr)),
        ]))
    }
}

// #[derive(Debug, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout)]
// #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
// struct Slot0 {
//     // the current price
//     sqrtPriceX96: Uint160,
//     // the current tick
//     tick: Int24,
//     // the most-recently updated index of the observations array
//     observationIndex: u16,
//     // the current maximum number of observations that are being stored
//     observationCardinality: u16,
//     // the next maximum number of observations to store, triggered in observations.write
//     observationCardinalityNext: u16,
//     // the current protocol fee as a percentage of the swap fee taken on withdrawal
//     // represented as an integer denominator (1/x)%
//     feeProtocol: u8,
//     // whether the pool is locked
//     unlocked: bool,
// }

// // accumulated protocol fees in token0/token1 units
// #[derive(Debug, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout)]
// #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
// struct ProtocolFees {
//     token0: u128,
//     token1: u128,
// }
