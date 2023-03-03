// Source: https://github.com/phi-labs-ltd/area-52-courses/blob/main/02_Do_Cyborgs_Dream_of_NFTs/nft/passport-token/src/lib.rs
// @see: https://docs.rs/cosmwasm-std/latest/cosmwasm_std/testing/index.html
#![cfg(test)]
mod tests {
    use crate::{Cw721MetadataContract, ExecuteMsg, Metadata};

    use cw721::Cw721Query;
    use cw721_soulbound::{InstantiateMsg, MintMsg};
    use universe::species::{Species, SapienceScale};

    use cosmwasm_std::Addr;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    const MINTER: &str = "jumpring";    // Each JumpRing mints passports and handles passport validation;
                                        // (Like airport security and an intergalactic embassy combined)

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let contract = Cw721MetadataContract::default();
        let info = mock_info(MINTER, &[]);

        let instantiate_msg = InstantiateMsg {
            name: "passport token".to_string(),
            symbol: "PASS".to_string(),
            minter: MINTER.to_string(),
        };

        contract.instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

        let species = Species {
            name: "Cyborg".to_string(),
            sapience_level: SapienceScale::High,
        };

        let metadata_extension = Some(Metadata {
            name: Some("Traveler Name".into()),
            description: Some("Ever since you became a cyborg, you've been feeling pretty weird...".into()),
            image: Some("ipfs://QmZdPdZzZum2jQ7jg1ekfeE3LSz1avAaa42G6mfimw9TEn".into()),
            dna: Some("Example DNA String".into()),
            species: Some(species.name),
            sapience_level: Some(species.sapience_level),
            issuer: Some(Addr::unchecked("archway1yvnw8xj5elngcq95e2n2p8f80zl7shfwyxk88858pl6cgzveeqtqy7xtf7")),
            origin: Some("earth".into()),
            identity: Some(Addr::unchecked("archway1f395p0gg67mmfd5zcqvpnp9cxnu0hg6r9hfczq")),
        });

        let token_id = "1".to_string();
        let mint_msg = MintMsg {
            token_id: token_id,
            owner: MINTER.to_string(),
            token_uri: None,
            extension: metadata_extension,
        };
        let execute_msg = ExecuteMsg::Mint(mint_msg.clone());
        contract.execute(deps.as_mut(), mock_env(), info, execute_msg).unwrap();

        let res = contract.nft_info(deps.as_ref(), token_id.into()).unwrap();

        assert_eq!(res.token_uri, mint_msg.token_uri); // off-chain metadata should be `None`
        assert_eq!(res.extension, mint_msg.extension); // on-chain metadata should be equal to `metadata_extension`
    }
}