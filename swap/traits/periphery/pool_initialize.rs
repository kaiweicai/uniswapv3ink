use openbrush::{
    traits::{
        AccountId,
    },
};
use primitives::{ Address, U160};

#[openbrush::wrapper]
pub type InitializerRef = dyn Initializer;


/// @title Creates and initializes V3 Pools
/// @notice Provides a method for creating and initializing a pool, if necessary, for bundling with other methods that
/// require the pool to exist.
#[openbrush::trait_definition]
pub trait Initializer{
    /// @notice Creates a new pool if it does not exist, then initializes if not initialized
    /// @dev This method can be bundled with others via IMulticall for the first action (e.g. mint) performed against a pool
    /// @param token0 The contract address of token0 of the pool
    /// @param token1 The contract address of token1 of the pool
    /// @param fee The fee amount of the v3 pool for the specified token pair
    /// @param sqrtPriceX96 The initial square root price of the pool as a Q64.96 value
    /// @return pool Returns the pool address based on the pair of tokens and fee, will return the newly created pool address if necessary
    #[ink(message, payable)]
    fn createAndInitializePoolIfNecessary(
        &mut self,
        token0: AccountId,
        token1: AccountId,
        fee: u32,
        sqrt_price_x96: U160,
    ) -> Address;
}