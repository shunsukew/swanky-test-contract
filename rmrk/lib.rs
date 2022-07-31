#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_env::Environment;
use scale::{Decode, Encode};
use ink_prelude::vec::Vec;
use ink_env::AccountId;

#[derive(Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RmrkErrorCode {
    Failed,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RmrkError {
    ErrorCode(RmrkErrorCode),
}

impl From<RmrkErrorCode> for RmrkError {
    fn from(error_code: RmrkErrorCode) -> Self {
        Self::ErrorCode(error_code)
    }
}

impl From<scale::Error> for RmrkError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}

impl ink_env::chain_extension::FromStatusCode for RmrkErrorCode {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::Failed),
            _ => panic!("encountered unknown status code"),
        }
    }
}

pub type CollectionId = u32;
pub type NftId = u32;
pub type ResourceId = u32;

/// Collection info.
#[derive(PartialEq, Debug, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct CollectionInfo {
	/// Current bidder and bid price.
	pub issuer: AccountId,

	pub metadata: Vec<u8>,
	pub max: Option<u32>,

	pub symbol: Vec<u8>,
	pub nfts_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize = <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = RmrkExt;
}

#[ink::chain_extension]
pub trait RmrkExt {
    type ErrorCode = RmrkErrorCode;

    // read
    #[ink(extension = 3501, returns_result = false, handle_status = false)]
    fn next_nft_id(collection_id: CollectionId) -> NftId;

    #[ink(extension = 3502, returns_result = false, handle_status = false)]
    fn collection_index() -> CollectionId;

    #[ink(extension = 3503, returns_result = false, handle_status = false)]
    fn next_resource_id() -> ResourceId;

    #[ink(extension = 3504)]
    fn collections(collection_id: CollectionId) -> Result<CollectionInfo, RmrkError>;

    // #[ink(extension = 3505)]
    // fn nfts(collection_id: u32, nft_id: u32) -> Result<InstanceInfo, RmrkError>;

    // #[ink(extension = 3508)]
    // fn resources(collection_id: u32, nft_id: u32, resource_id: u32) -> Result<ResourceInfo, RmrkError>;

    // write
    // #[ink(extension = 3513, returns_result = false)]
    // fn mint_nft(
        // owner: AccountId,
        // collection_id: CollectionId,
        // royalty_recipient: Option<AccountId>,
        // royalty: Option<Permill>,
        // metadata: Vec<u8>,
        // transferable: bool,
        // resources: Option<Vec<>>
    // ); 

    #[ink(extension = 3515, returns_result = false)]
    fn create_collection(
        metadata: Vec<u8>,
        max: Option<u32>,
        symbol: Vec<u8>,
    );
}

#[ink::contract(env = crate::CustomEnvironment)]
mod rmrk {
    use super::*;

    #[ink(storage)]
    pub struct RmrkTestContract {}

    impl RmrkTestContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            RmrkTestContract {}
        }

        #[ink(message)]
        pub fn next_nft_id(&self, collection_id: CollectionId) -> u32 {
            self.env().extension().next_nft_id(collection_id)
        }

        #[ink(message)]
        pub fn collection_index(&self) -> CollectionId {
            self.env().extension().collection_index()
        }

        #[ink(message)]
        pub fn next_resource_id(&self) -> ResourceId {
            self.env().extension().next_resource_id()
        }

        #[ink(message)]
        pub fn collections(&self, collection_id: CollectionId) -> Result<CollectionInfo, RmrkError> {
            self.env().extension().collections(collection_id)
        }

        /// write
        #[ink(message)]
        pub fn create_collection(&mut self) -> Result<(), RmrkErrorCode> {
            let metadata = "ipfs://ipfs/QmTG9ekqrdMh3dsehLYjC19fUSmPR31Ds2h6Jd7LnMZ9c7";
            let symbol = "ROO";
            let max = Some(1000);

            self.env().extension().create_collection(
                metadata.into(),
                max,
                symbol.into(),
            )
        }
    }

    #[cfg(test)]
    mod tests {
    }
}
