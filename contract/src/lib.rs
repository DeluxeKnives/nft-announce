use interfaces::{mintbase_store, TokenCompliant};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U64;
use near_sdk::{env, near_bindgen, AccountId, Gas, PanicOnDefault, Promise, PromiseError};
mod interfaces;

const TGAS: u64 = 1_000_000_000_000;

// Defines contract data structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NFTAnnounce {
    pub nft_store_contract: AccountId,
    pub announcements: UnorderedMap<U64, String>,
}

// Implement the contract structure
#[near_bindgen]
impl NFTAnnounce {
    #[init]
    pub fn new(nft_store_contract: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        // TODO: no clue what the prefix does for the UnorderedMap
        Self {
            nft_store_contract,
            announcements: UnorderedMap::new(b"d"),
        }
    }

    // Public method - returns the account ID of the nft store contract
    pub fn get_nft_store_contract(&self) -> AccountId {
        self.nft_store_contract.clone()
    }

    // Public method - returns an announcement for a specific string
    pub fn get_announcement(&self, nft_id: U64) -> String {
        return match self.announcements.get(&nft_id) {
            Some(announcement) => announcement,
            None => "".to_string(),
        };
    }

    // Public method - returns an announcement for a specific string
    pub fn get_nft_owner(&self, nft_id: U64) -> Promise {
        let nft_data_promise = mintbase_store::ext(self.nft_store_contract.clone())
            .with_static_gas(Gas(5 * TGAS))
            .nft_token(nft_id);
        return nft_data_promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(Gas(5 * TGAS))
                .query_token_view_callback(),
        );
    }

    // Public method - accepts a string if the user owns the specified nft
    pub fn announce(&mut self, nft_id: U64, announcement: String) -> Promise {
        let nft_data_promise = mintbase_store::ext(self.nft_store_contract.clone())
            .with_static_gas(Gas(5 * TGAS))
            .nft_token(nft_id);

        nft_data_promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(Gas(5 * TGAS))
                .query_token_callback(nft_id, announcement),
        )
    }

    #[private]
    pub fn query_token_callback(
        &mut self,
        #[callback_result] call_result: Result<TokenCompliant, PromiseError>,
        nft_id: U64,
        announcement: String,
    ) {
        // Check if the promise succeeded
        if call_result.is_err() {
            panic!("There was an error contacting the NFT contract!");
        }

        // Get the token
        let token: TokenCompliant = call_result.unwrap();

        // Ensure that nothing happens
        if token.owner_id == env::signer_account_id() {
            self.announcements.insert(&nft_id, &announcement);
        }
        else {
            panic!("Only owners can announce!");
        }
    }

    #[private]
    pub fn query_token_view_callback(
        self,
        #[callback_result] call_result: Result<TokenCompliant, PromiseError>,
    ) -> AccountId {
        // Check if the promise succeeded
        if call_result.is_err() {
            panic!("There was an error contacting the NFT contract!");
        }

        // Get the token
        let token: TokenCompliant = call_result.unwrap();
        token.owner_id
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_nft_contract() {
        let alice: AccountId = "alice.near".parse().unwrap();
        assert!("invalid.".parse::<AccountId>().is_err());

        let announcer = NFTAnnounce {
            nft_store_contract: alice.clone(),
            announcements: UnorderedMap::new(b"d"),
        };
        let store = announcer.get_nft_store_contract();

        assert_eq!(store, alice);
    }

    #[test]
    fn set_hash_as_owner() {}

    #[test]
    fn try_set_hash_not_owner() {}
}
