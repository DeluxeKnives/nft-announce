use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, env, AccountId};
use near_sdk::collections::UnorderedMap;

// Defines contract data structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NFTAnnounce {
    pub nft_contract: AccountId,
    pub announcements: UnorderedMap<u64, String>,
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
    pub fn init(nft_contract: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        // TODO: no clue what the prefix does for the UnorderedMap
        Self {
            nft_contract,
            announcements: UnorderedMap::new(b"d")
        }
    }

    // Public method - returns an announcement for a specific string
    pub fn get_announcement(&self, nft_id: u64) -> String {
        return match self.announcements.get(&nft_id) {
            Some(announcement) => announcement,
            None => "".to_string()
        };
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn announce(&mut self, nft_id: u64, announcement: String) {
        // TODO: check to make sure that the message sender owns the nft of nft_id
        let caller = env::predecessor_account_id();

        // TODO: get dependencies by adding a git submodule for https://github.com/near-examples/nft-tutorial/blob/4.core/nft-contract/src/nft_core.rs
        let promise = hello_near::ext(self.nft_contract.clone())
            .with_static_gas(Gas(5*TGAS))
            .get_greeting();

        self.announcements.insert(&nft_id, &announcement);
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

    }

    #[test]
    fn set_hash_as_owner() {

    }

    #[test]
    fn try_set_hash_not_owner() {

    }
}
