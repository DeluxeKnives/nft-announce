use std::collections::HashMap;

use near_sdk::{AccountId, json_types::{Base64VecU8, U64}, ext_contract};

#[ext_contract(mintbase_store)]
pub trait MintbaseStore {
    fn nft_token(
        &self,
        token_id: U64,
    ) -> Option<TokenCompliant>;
}

pub struct TokenCompliant {
    /// The id of this token on this `Store`. Not unique across `Store`s.
    /// `token_id`s count up from 0. Ref: https://github.com/near/NEPs/discussions/171
    pub token_id: String,
    /// The current owner of this token. Either an account_id or a token_id (if composed).
    pub owner_id: Owner,
    /// Ref:
    /// https://github.com/near/NEPs/blob/master/specs/Standards/NonFungibleToken/ApprovalManagement.md
    /// Set of accounts that may transfer this token, other than the owner.
    pub approved_account_ids: HashMap<AccountId, u64>,
    /// The metadata content for this token is stored in the Contract
    /// `token_metadata` field, to avoid duplication of metadata across tokens.
    /// Use metadata_id to lookup the metadata. `Metadata`s is permanently set
    /// when the token is minted.
    pub metadata: TokenMetadataCompliant,
    /// The Royalty for this token is stored in the Contract `token_royalty`
    /// field, to avoid duplication across tokens. Use royalty_id to lookup the
    /// royalty. `Royalty`s are permanently set when the token is minted.
    pub royalty: Option<Royalty>,
    /// Feature for owner of this token to split the token ownership accross
    /// several accounts.
    pub split_owners: Option<SplitOwners>,
    /// The account that minted this token.
    pub minter: AccountId,
    /// Non-nil if Token is loaned out. While token is loaned, disallow
    /// transfers, approvals, revokes, etc. for the token, except from the
    /// approved loan contract. Mark this field with the address of the loan
    /// contract. See neps::loan for more.
    pub loan: Option<Loan>,
    /// Composeablility metrics for this token
    pub composeable_stats: ComposeableStats,
    /// If the token originated on another contract and was `nft_move`d to
    /// this contract, this field will be non-nil.
    pub origin_key: Option<TokenKey>,
}

pub struct Loan {
    pub holder: AccountId,
    pub loan_contract: AccountId,
}

pub struct ComposeableStats {
    /// How deep this token is in a chain of composeability on THIS contract.
    /// If this token is cross-composed, it's depth will STILL be 0. `depth`
    /// equal to the parent's `depth`+1. If this is a top level token, this
    /// number is 0.
    pub local_depth: u8,
    /// How many cross contract children this token has, direct AND indirect.
    /// That is, any parent's `cross_contract_children` value equals the sum
    /// of of its children's values. If this number is non-zero, deny calls
    /// to `nft_cross_compose`.
    pub cross_contract_children: u8,
}

pub enum Owner {
    /// Standard pattern: owned by a user.
    Account(AccountId),
    /// Compose pattern: owned by a token on this contract.
    TokenId(u64),
    /// Cross-compose pattern: owned by a token on another contract.
    CrossKey(TokenKey),
    /// Lock: temporarily locked until some callback returns.
    Lock(AccountId),
}

pub struct TokenKey {
    pub token_id: u64,
    pub account_id: String,
}

pub struct TokenMetadataCompliant {
    /// the Title for this token. ex. "Arch Nemesis: Mail Carrier" or "Parcel 5055"
    pub title: Option<String>,
    /// free-form description of this token.
    pub description: Option<String>,
    /// URL to associated media, preferably to decentralized, content-addressed storage
    pub media: Option<String>,
    /// Base64-encoded sha256 hash of content referenced by the `media` field.
    /// Required if `media` is included.
    pub media_hash: Option<Base64VecU8>,
    /// number of copies of this set of metadata in existence when token was minted.
    pub copies: Option<u16>,
    /// When token was issued or minted, Unix epoch in milliseconds
    pub issued_at: Option<String>,
    /// ISO 8601 datetime when token expires.
    pub expires_at: Option<String>,
    /// ISO 8601 datetime when token starts being valid.
    pub starts_at: Option<String>,
    /// When token was last updated, Unix epoch in milliseconds
    pub updated_at: Option<String>,
    /// Brief description of what this thing is. Used by the mintbase indexer as "memo".
    pub extra: Option<String>,
    /// URL to an off-chain JSON file with more info. The Mintbase Indexer refers
    /// to this field as `thing_id` or sometimes, `meta_id`.
    pub reference: Option<String>,
    /// Base64-encoded sha256 hash of JSON from reference field. Required if
    /// `reference` is included.
    pub reference_hash: Option<Base64VecU8>,
}

pub struct Royalty {
    /// Mapping of addresses to relative percentages of the overall royalty percentage
    pub split_between: HashMap<near_sdk::AccountId, SafeFraction>,
    /// The overall royalty percentage taken
    pub percentage: SafeFraction,
}

pub struct SplitOwners {
    pub split_between: HashMap<AccountId, SafeFraction>,
}

pub struct SafeFraction {
    pub numerator: u32,
}
