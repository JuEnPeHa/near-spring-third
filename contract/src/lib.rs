
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, assert_one_yocto, ext_contract, AccountId, Balance,
    Gas, PanicOnDefault, Promise, CryptoHash, BorshStorageKey, PromiseOrValue, promise_result_as_success, require};
use near_sdk::collections::{LookupMap, UnorderedMap, LazyOption, UnorderedSet};
use std::collections::HashMap;
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;
pub use crate::events::*;

mod internal;
mod approval; 
mod enumeration; 
mod metadata; 
mod mint; 
mod nft_core; 
mod royalty; 
mod events;

pub const NFT_METADATA_SPEC: &str = "1.0.0";
pub const NFT_STANDARD_NAME: &str = "nep171";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
    pub tokens_by_id: LookupMap<TokenId, Token>,
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,
    pub metadata: LazyOption<NFTContractMetadata>,
    pub records: UnorderedMap<AccountId, String>,
    pub minted: UnorderedMap<u64, Token>,
}

#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "JEPH NFT Contract".to_string(),
                symbol: "JEPH".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        let this = Self {
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            records: UnorderedMap::new(b"a".to_vec()),
            minted: UnorderedMap::new(b"b".to_vec()),
        };

        this
    }

    pub fn set_greeting(&mut self, message: String) {
        let account_id = env::signer_account_id();

        env::log_str(format!("Saving greeting '{}' for account '{}'", message.as_str(), &account_id.as_str(),).as_str());

        if self.records.is_empty() {
            self.records = UnorderedMap::new(b"a".to_vec());
        }
            self.records.insert(&account_id, &message);
    }


    pub fn get_greeting(&self, account_id: AccountId) -> String {
        match self.records.get(&account_id) {
            Some(greeting) => greeting,
            None => "Hello".to_string(),
        }
    }

    pub fn get_all_greetings(&mut self) -> Vec<String> {
        self.records.insert(&AccountId::new_unchecked("jeph.testnet".to_string()),&"HONOLULU".to_string());
        let mut next: Vec<String> = self.records.iter().map(|(_, v)| v.clone()).collect();
        next.insert(0, "Hello".to_string());
        next
    }
}