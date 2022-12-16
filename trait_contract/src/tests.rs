#[cfg(test)]
mod tests {
    use crate::{TraitCollection, TraitContract, TraitContractClient, AssetTrait, TraitOptionItem, TraitOptionValue, TRAITS};
    use soroban_sdk::{bytes, symbol, vec, Env, Vec};

    fn get_client() -> TraitContractClient {
        let env = Env::default();
        let cid = env.register_contract(None, TraitContract);
        TraitContractClient::new(&env, &cid)
    }

    #[test]
    #[should_panic(expected = "Status(ContractError(1))")] // Error::AlreadyInitialized
    fn init() {
        let client = get_client();
        let env = &client.env;
        // "test".hex => 74657374
        assert_eq!(client.init(&bytes!(env, 0x74657374), &10), TraitCollection{name: bytes!(env, 0x74657374), size: 10});
        // "other".hex => 6f74686572
        client.init(&bytes!(env, 0x6f74686572), &1);
    }

    #[test]
    #[should_panic(expected = "Status(ContractError(0))")] // Error::NotInitialized
    fn finalize_uninitialized() {
        let client = get_client();
        client.finalize();
    }

    #[test]
    fn finalize_initialized() {
        let client = get_client();
        let env = &client.env;
        // "test".hex => 74657374
        client.init(&bytes!(env, 0x74657374), &1);
        assert!(client.finalize());
    }

    #[test]
    fn add_trait() {
        let client = get_client();
        let env = &client.env;

        let new_trait = AssetTrait{
            name: symbol!("trait_1"),
            // "This is the 1st trait.".hex => 5468697320697320746865203173742074726169742e
            desc: bytes!(env, 0x5468697320697320746865203173742074726169742e),
            options: vec![env], //, TraitOptionItem{name: symbol!("option1"), value: TOV::Numeric(100), available: 1}],
        };

        let traits = client.add_trait(&new_trait.name, &new_trait.desc);
        assert_eq!(
            traits,
            vec![env, new_trait]
        );
    }

    #[test]
    fn add_options() {
        let client = get_client();
        let env = &client.env;

        client.init(&bytes!(&env, 0xff), &10);
        // "a trait with options".hex => 612074726169742077697468206f7074696f6e73
        _ = client.add_trait(&symbol!("has_opts"), &bytes!(env, 0x612074726169742077697468206f7074696f6e73));

        _ = client.add_option(&symbol!("has_opts"), &symbol!("option_1"), &TraitOptionValue::Numeric(1));
        _ = client.add_option(&symbol!("has_opts"), &symbol!("option_2"), &TraitOptionValue::Numeric(5));
        let res = client.add_option(&symbol!("has_opts"), &symbol!("option_3"), &TraitOptionValue::Numeric(10));

        let updated_trait = env
            .as_contract(&client.contract_id, || env.storage().get::<_, Vec<AssetTrait>>(TRAITS))
            .expect("contract must have traits")
            .unwrap()
            .get(0)
            .expect("must be some")
            .expect("must be a trait");

        assert_eq!(
            updated_trait.options,
            res.options
        );
        assert!(client.finalize());

        let finalized_trait = env
        .as_contract(&client.contract_id, || env.storage().get::<_, Vec<AssetTrait>>(TRAITS))
        .expect("contract must have traits")
        .unwrap()
        .get(0)
        .expect("must be some")
        .expect("must be a trait");

        assert!(finalized_trait.options.get_unchecked(0).unwrap().available > 0);
    }


    #[test]
    #[should_panic(expected= "Status(ContractError(3)")] // Error::TraitNotFound
    fn add_option_to_non_existent_trait() {
        let client = get_client();
        let env = &client.env;

        let option = TraitOptionItem{
            name: symbol!("option_1"), value: TraitOptionValue::Numeric(1), available: 0
        };

        let res = client.add_option(&symbol!("has_opts"), &symbol!("option_1"), &TraitOptionValue::Numeric(1));
        assert_eq!(
            res.options,
            vec![&env, option]
        );
    }
}
