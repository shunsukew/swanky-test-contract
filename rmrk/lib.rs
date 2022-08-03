#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::AccountId;
use ink_env::Environment;
use ink_lang as ink;
use ink_prelude::vec::Vec;
use scale::{Decode, Encode};

mod types;

use types::*;

#[cfg(test)]
mod mock;

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

    // READ functions
    #[ink(extension = 3501, returns_result = false, handle_status = false)]
    fn next_nft_id(collection_id: CollectionId) -> NftId;

    #[ink(extension = 3502, returns_result = false, handle_status = false)]
    fn collection_index() -> CollectionId;

    #[ink(extension = 3503, returns_result = false, handle_status = false)]
    fn next_resource_id(collection_id: CollectionId, nft_id: NftId) -> ResourceId;

    #[ink(extension = 3504)]
    fn collections(collection_id: CollectionId) -> Result<Option<CollectionInfo>, RmrkError>;

    // #[ink(extension = 3505)]
    // fn nfts(collection_id: CollectionId, nft_id: NftId) -> Result<NftInfo, RmrkError>;

    #[ink(extension = 3506, returns_result = false, handle_status = false)]
    fn priorities(collection_id: CollectionId, nft_id: NftId, resource_id: ResourceId) -> u32;

    // #[ink(extension = 3507)]
    // fn children() -> Result<>;

    #[ink(extension = 3508)]
    fn resources(
        collection_id: u32,
        nft_id: NftId,
        resource_id: ResourceId,
    ) -> Result<ResourceInfo, RmrkError>;

    #[ink(extension = 3509)]
    fn equippable_bases(
        collection_id: CollectionId,
        nft_id: NftId,
        base_id: BaseId,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3510)]
    fn equippable_slots(
        collection_id: CollectionId,
        nft_id: NftId,
        resource_id: ResourceId,
        base_id: BaseId,
        slot_id: SlotId,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3511)]
    fn properties(
        collection_id: CollectionId,
        nft_id: NftId,
        resource_id: ResourceId,
    ) -> Result<Vec<u8>, RmrkError>;

    #[ink(extension = 3512)]
    fn lock(collection_id: CollectionId, nft_id: NftId) -> Result<bool, RmrkError>;

    // WRITE functions
    #[ink(extension = 3513)]
    fn mint_nft(
        owner: AccountId,
        collection_id: u32,
        royalty_recipient: Option<AccountId>,
        royalty: Option<u8>,
        metadata: Vec<u8>,
        transferable: bool,
        resources: Option<((Vec<u8>, Vec<u8>), u32)>,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3514)]
    fn mint_nft_directly_to_nft(
        owner: (CollectionId, NftId),
        collection_id: u32,
        royalty_recipient: Option<AccountId>,
        royalty: Option<u8>,
        metadata: Vec<u8>,
        transferable: bool,
        resources: Option<((Vec<u8>, Vec<u8>), u32)>,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3515)]
    fn create_collection(
        metadata: Vec<u8>,
        max: Option<u32>,
        symbol: Vec<u8>,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3516)]
    fn burn_nft(
        collection_id: CollectionId,
        nft_id: NftId,
        max_burns: u32,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3517)]
    fn destroy_collection(collection_id: CollectionId) -> Result<(), RmrkError>;

    #[ink(extension = 3518)]
    fn send(
        collection_id: CollectionId,
        nft_id: NftId,
        new_owner: AccountIdOrCollectionNftTuple,
    ) -> Result<(), RmrkError>;
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

        // READ functions
        #[ink(message)]
        pub fn next_nft_id(&self, collection_id: CollectionId) -> u32 {
            self.env().extension().next_nft_id(collection_id)
        }

        #[ink(message)]
        pub fn collection_index(&self) -> CollectionId {
            self.env().extension().collection_index()
        }

        #[ink(message)]
        pub fn next_resource_id(&self, collection_id: CollectionId, nft_id: NftId) -> ResourceId {
            self.env()
                .extension()
                .next_resource_id(collection_id, nft_id)
        }

        #[ink(message)]
        pub fn collections(
            &self,
            collection_id: CollectionId,
        ) -> Result<Option<CollectionInfo>, RmrkError> {
            self.env().extension().collections(collection_id)
        }

        #[ink(message)]
        pub fn priorities(
            &self,
            collection_id: CollectionId,
            nft_id: NftId,
            resource_id: ResourceId,
        ) -> u32 {
            self.env()
                .extension()
                .priorities(collection_id, nft_id, resource_id)
        }

        #[ink(message)]
        pub fn resources(
            &self,
            collection_id: CollectionId,
            nft_id: NftId,
            resource_id: ResourceId,
        ) -> Result<ResourceInfo, RmrkError> {
            self.env()
                .extension()
                .resources(collection_id, nft_id, resource_id)
        }

        #[ink(message)]
        pub fn equippable_bases(
            &self,
            collection_id: CollectionId,
            nft_id: NftId,
            base_id: BaseId,
        ) -> Result<(), RmrkError> {
            self.env()
                .extension()
                .equippable_bases(collection_id, nft_id, base_id)
        }

        #[ink(message)]
        pub fn equippable_slots(
            &self,
            collection_id: CollectionId,
            nft_id: NftId,
            resource_id: ResourceId,
            base_id: BaseId,
            slot_id: SlotId,
        ) -> Result<(), RmrkError> {
            self.env().extension().equippable_slots(
                collection_id,
                nft_id,
                resource_id,
                base_id,
                slot_id,
            )
        }

        #[ink(message)]
        pub fn properties(
            &self,
            collection_id: CollectionId,
            nft_id: NftId,
            resource_id: ResourceId,
        ) -> Result<Vec<u8>, RmrkError> {
            self.env()
                .extension()
                .properties(collection_id, nft_id, resource_id)
        }

        #[ink(message)]
        pub fn lock(&self, collection_id: CollectionId, nft_id: NftId) -> Result<bool, RmrkError> {
            self.env().extension().lock(collection_id, nft_id)
        }

        /// write functions
        #[ink(message)]
        pub fn mint_ntf(
            &mut self,
            owner: AccountId,
            collection_id: u32,
            royalty_recipient: Option<AccountId>,
            _royalty: Option<u8>,
            metadata: Vec<u8>,
            transferable: bool,
            resources: Option<((Vec<u8>, Vec<u8>), u32)>,
        ) -> Result<(), RmrkError> {
            self.env().extension().mint_nft(
                owner,
                collection_id,
                royalty_recipient,
                None, // fix to use Permill
                metadata,
                transferable,
                resources,
            )
        }

        #[ink(message)]
        pub fn mint_ntf_directly_to_nft(
            &mut self,
            owner: (CollectionId, NftId),
            collection_id: u32,
            royalty_recipient: Option<AccountId>,
            _royalty: Option<u8>,
            metadata: Vec<u8>,
            transferable: bool,
            resources: Option<((Vec<u8>, Vec<u8>), u32)>,
        ) -> Result<(), RmrkError> {
            self.env().extension().mint_nft_directly_to_nft(
                owner,
                collection_id,
                royalty_recipient,
                None, // fix to use Permill
                metadata,
                transferable,
                resources,
            )
        }

        #[ink(message)]
        pub fn create_collection(&mut self) -> Result<(), RmrkError> {
            let metadata = "ipfs://ipfs/QmTG9ekqrdMh3dsehLYjC19fUSmPR31Ds2h6Jd7LnMZ9c7";
            let symbol = "ROO";
            let max = Some(1000);

            self.env()
                .extension()
                .create_collection(metadata.into(), max, symbol.into())
        }

        #[ink(message)]
        pub fn burn_nft(
            &mut self,
            collection_id: CollectionId,
            nft_id: NftId,
            max_burns: u32,
        ) -> Result<(), RmrkError> {
            self.env()
                .extension()
                .burn_nft(collection_id, nft_id, max_burns)
        }

        #[ink(message)]
        pub fn destroy_collection(&mut self, collection_id: CollectionId) -> Result<(), RmrkError> {
            self.env().extension().destroy_collection(collection_id)
        }

        #[ink(message)]
        pub fn send(
            &mut self,
            collection_id: CollectionId,
            nft_id: NftId,
            new_owner: AccountIdOrCollectionNftTuple,
        ) -> Result<(), RmrkError> {
            self.env()
                .extension()
                .send(collection_id, nft_id, new_owner)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_env::test;

        fn init_test_contract() -> RmrkTestContract {
            let test_contract = RmrkTestContract::new();

            // mock::register_chain_extension();

            test_contract
        }

        fn accounts() -> test::DefaultAccounts<CustomEnvironment> {
            test::default_accounts()
        }

        fn alice() -> AccountId {
            accounts().alice
        }

        fn bob() -> AccountId {
            accounts().bob
        }
    }
}
