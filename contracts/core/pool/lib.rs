#![cfg_attr(not(feature = "std"), no_std)]

// SPDX-License-Identifier: GPL-2.0-or-later
use ink_lang as ink;

pub use self::uniswap_v3_pool::{UniswapV3Pool, UniswapV3PoolRef};

#[ink::contract]
mod uniswap_v3_pool {
    use ink_storage::{
        Mapping,
        traits::{PackedLayout, SpreadLayout, StorageLayout},
    };
    use scale::{Decode, Encode, WrapperTypeEncode};
    use primitives::Uint160;
    #[cfg(feature = "std")]
    use ink_metadata::layout::{FieldLayout, Layout, StructLayout};

    type Address = AccountId;
    type Uint24 = u32;
    type Int24 = i32;

    #[derive(Debug, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    struct Slot0 {
        // the current price
        sqrtPriceX96: Uint160,
        // the current tick
        tick: Int24,
        // the most-recently updated index of the observations array
        observationIndex: u16,
        // the current maximum number of observations that are being stored
        observationCardinality: u16,
        // the next maximum number of observations to store, triggered in observations.write
        observationCardinalityNext: u16,
        // the current protocol fee as a percentage of the swap fee taken on withdrawal
        // represented as an integer denominator (1/x)%
        feeProtocol: u8,
        // whether the pool is locked
        unlocked: bool,
    }

    // accumulated protocol fees in token0/token1 units
    #[derive(Debug, PartialEq, Eq, Encode, Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    struct ProtocolFees {
        token0: u128,
        token1: u128,
    }

    #[ink(storage)]
    pub struct UniswapV3Pool {
        // address public immutable override factory;
        // address public immutable override token0;
        // address public immutable override token1;
        // uint24 public immutable override fee;
        // int24 public immutable override tickSpacing;
        // uint128 public immutable override maxLiquidityPerTick;

        // follow six parameter is immutable
        pub factory: Address,
        pub token0: Address,
        pub token1: Address,
        pub fee: Uint24,
        pub tick_spacing: Int24,
        pub max_liquidity_per_tick: u128,
        // pub slot0: Slot0,

        pub fee_growth_global0_x128: Uint160,
        pub fee_growth_global1_x128: Uint160,

        // pub protocolFees: ProtocolFees,

        pub liquidity: u128,
        // mapping(int24 => Tick.Info) pub ticks;
        // /// @inheritdoc IUniswapV3PoolState
        // mapping(int16 => uint256) public override tickBitmap;
        // /// @inheritdoc IUniswapV3PoolState
        // mapping(bytes32 => Position.Info) public override positions;
    }

    impl Default for UniswapV3Pool {
        fn default() -> Self {
            Self {
                factory: Default::default(),
                token0: Default::default(),
                token1: Default::default(),
                fee: Default::default(),
                tick_spacing: Default::default(),
                max_liquidity_per_tick: Default::default(),
                fee_growth_global0_x128: Uint160{value:0},
                fee_growth_global1_x128: Uint160{value:0},
                // protocolFees: WrapperU256{value:U256([0u64;4])},
                liquidity: Default::default(),
            }
        }
    }

    impl UniswapV3Pool {
        #[ink(constructor)]
        pub fn new(factory:Address,token0: Address, token1: Address, fee: Uint24, tick_spacing: Int24) -> Self {
            // (factory, token0, token1, fee, _tickSpacing) = IUniswapV3PoolDeployer(msg.sender).parameters();
            // tickSpacing = _tickSpacing;
            // maxLiquidityPerTick = Tick.tickSpacingToMaxLiquidityPerTick(_tickSpacing);

            let instance = Self{
                factory,
                token0,
                token1,
                fee,
                tick_spacing,
                max_liquidity_per_tick: Default::default(),
                fee_growth_global0_x128: Uint160{value:0},
                fee_growth_global1_x128: Uint160{value:0},
                // protocolFees: WrapperU256{value:U256([0u64;4])},
                liquidity: Default::default(),
            };
            instance
        }

        /// @inheritdoc IUniswapV3Factory
        #[ink(message)]
        pub fn create_pool(&mut self, tokenA: Address, tokenB: Address, fee: u32) -> AccountId {
            [0; 32].into()
        }
    }
}
