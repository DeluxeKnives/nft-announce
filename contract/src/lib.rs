use interfaces::{mintbase_store, TokenCompliant, Owner};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U64;
use near_sdk::{env, near_bindgen, AccountId, Gas, PromiseError};
mod interfaces;

const TGAS: u64 = 1_000_000_000_000;

// Defines contract data structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NFTAnnounce {
    pub nft_store_contract: AccountId,
    pub announcements: UnorderedMap<U64, String>,
}

// Don't allow initialization without constructors
impl Default for NFTAnnounce {
    fn default() -> Self {
        env::panic_str("Not initialized yet.");
    }
}

// Implement the contract structure
#[near_bindgen]
impl NFTAnnounce {
    #[init]
    #[private]
    pub fn init(nft_store_contract: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        // TODO: no clue what the prefix does for the UnorderedMap
        Self {
            nft_store_contract,
            announcements: UnorderedMap::new(b"d"),
        }
    }

    // Public method - returns an announcement for a specific string
    pub fn get_announcement(&self, nft_id: U64) -> String {
        return match self.announcements.get(&nft_id) {
            Some(announcement) => announcement,
            None => "".to_string(),
        };
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn announce(&mut self, nft_id: U64, announcement: String) {
        let nft_data_promise = mintbase_store::ext(self.nft_store_contract.clone())
            .with_static_gas(Gas(5 * TGAS))
            .nft_token(nft_id);

        nft_data_promise.then(
            Self::ext(env::predecessor_account_id())
                .with_static_gas(Gas(5 * TGAS))
                .query_token_callback(nft_id, announcement),
        );
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn query_token_callback(
        &mut self,
        #[callback_result] call_result: Result<TokenCompliant, PromiseError>,
        nft_id: U64,
        announcement: String
    ) {
        // Check if the promise succeeded
        if call_result.is_err() {
            panic!("There was an error contacting the NFT contract!");
        }

        // Get the token
        let token: TokenCompliant = call_result.unwrap();

        // Ensure that nothing happens
        match token.owner_id {
            Owner::Account(owner) => {
                if owner == env::predecessor_account_id() {
                    self.announcements.insert(&nft_id, &announcement);
                }
            },
            _ => panic!("Only accounts can announce!"),
        }
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
    fn get_nft_contract() {}

    #[test]
    fn set_hash_as_owner() {}

    #[test]
    fn try_set_hash_not_owner() {}
}
