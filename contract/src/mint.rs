use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        //token_id: TokenId,
        //metadata: TokenMetadata,
        //receiver_id: AccountId,
        //we add an optional parameter for perpetual royalties
        //perpetual_royalties: Option<HashMap<AccountId, u32>>,
    ) {
        let receiver_id = env::signer_account_id();
        let mut token_id = 0 as u64;
        if self.minted.is_empty() {
            token_id = 0;
        } else {
            token_id = self.minted.len() as u64;
        }

        let mut description = "Hey, you have a new token! ".to_string();
        let description_account = env::signer_account_id().to_string();
        let description_space = " and is the number ".to_string();
        let description_token = token_id.to_string();
        description.push_str(&description_account);
        description.push_str(&description_space);
        description.push_str(&description_token);

        let token_id_str = token_id.clone().to_string();
 
        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        // create a royalty map to store in the token
        let royalty = HashMap::new();

        //specify the token struct that contains the owner ID 
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id_str, &token).is_none(),
            "Token already exists"
        );

        let metadata = TokenMetadata {
            title: Some("JEPH NFT Contract".to_string()),
            description: Some(description),
            media: Some("https://gateway.pinata.cloud/ipfs/QmTJcDggLZEAYckUaPefTCdJzfL8eNBGvQTxiXyDbpfYvj".to_string()),
            media_hash: None,
            reference: None,
            reference_hash: None,
            copies: Some(1),
            issued_at: Some(env::block_timestamp()),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
        };

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id_str, &metadata);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id_str);

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);

        self.minted.insert(&token_id, &token);
    }
}