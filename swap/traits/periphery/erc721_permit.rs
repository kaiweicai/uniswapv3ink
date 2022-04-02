use brush::{
    traits::{
        AccountId,
    },
};
use primitives::U256;
use ink_prelude::string::String;


#[brush::wrapper]
pub type ERC721PermitRef = dyn IERC721Permit;


/// @title Creates and initializes V3 Pools
/// @notice Provides a method for creating and initializing a pool, if necessary, for bundling with other methods that
/// require the pool to exist.
#[brush::trait_definition]
pub trait IERC721Permit{
    /// @notice The permit typehash used in the permit signature
    /// @return The typehash for the permit
    #[ink(message)]
    fn PERMIT_TYPEHASH(&self) ->String;

    /// @notice The domain separator used in the permit signature
    /// @return The domain seperator used in encoding of permit signature
    #[ink(message)]
    fn DOMAIN_SEPARATOR(&self) -> [u8;32];

    /// @notice Approve of a specific token ID for spending by spender via signature
    /// @param spender The account that is being approved
    /// @param tokenId The ID of the token that is being approved for spending
    /// @param deadline The deadline timestamp by which the call must be mined for the approve to work
    /// @param v Must produce valid secp256k1 signature from the holder along with `r` and `s`
    /// @param r Must produce valid secp256k1 signature from the holder along with `v` and `s`
    /// @param s Must produce valid secp256k1 signature from the holder along with `r` and `v`
    #[ink(message, payable)]
    fn permit(
        &self,
        spender:AccountId,
        token_id:U256,
        deadline:U256,
        v:u8,
        r:String,
        s:String
    );
}