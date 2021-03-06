use openbrush::{
    declare_storage_trait,
};
use openbrush::traits::AccountId;
use ink_storage::traits::{SpreadAllocate, SpreadLayout,StorageLayout};

use primitives::Address;
use crate::traits::periphery::periphery_immutable_state::PeripheryImmutableState;
pub use swap_project_derive::ImmutableStateStorage;

#[derive(Default, Debug, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
/// define the struct with the data that our smart contract will be using
/// this will isolate the logic of our smart contract from its storage
pub struct ImmutableStateData {
    /// @inheritdoc IPeripheryImmutableState
    pub factory:AccountId,
    /// @inheritdoc IPeripheryImmutableState
    pub WETH9:AccountId,
}

declare_storage_trait!(ImmutableStateStorage);

impl<T:ImmutableStateStorage<Data = ImmutableStateData>> PeripheryImmutableState for T{
    /// @return Returns the address of the Uniswap V3 factory
    default fn factory(&self) -> Address{
        self.get().factory
    }

    /// @return Returns the address of WETH9
    default fn WETH9(&self) -> Address{
        self.get().WETH9
    }
}