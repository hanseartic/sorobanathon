#[cfg(test)]
mod tests {
    use crate::{TraitCollection, TraitContract, TraitContractClient, AssetTrait, TraitOptionItem, TraitOptionValue};
    use crate::{ASSIGNED, TRAITS};
    use soroban_sdk::{bytes, symbol, testutils, vec, BytesN, Env, Map, Symbol, Vec};

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

        client.init(&bytes!(&env, 0xff), &1);
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
        client.add_trait(&symbol!("has_opts"), &bytes!(env, 0x612074726169742077697468206f7074696f6e73));

        client.add_option(&symbol!("has_opts"), &symbol!("option_1"), &TraitOptionValue::Numeric(1));
        client.add_option(&symbol!("has_opts"), &symbol!("option_2"), &TraitOptionValue::Numeric(5));
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

        for o in finalized_trait.options {
            assert!(o.unwrap().available > 0);
        }
    }

    #[test]
    #[should_panic(expected= "Status(ContractError(3)")] // Error::TraitNotFound
    fn add_option_to_non_existent_trait() {
        let client = get_client();
        let env = &client.env;

        let option = TraitOptionItem{
            name: symbol!("option_1"), value: TraitOptionValue::Numeric(1), available: 0, total: 0,
        };

        client.init(&bytes!(&env, 0xff), &1);
        let res = client.add_option(&symbol!("has_opts"), &symbol!("option_1"), &TraitOptionValue::Numeric(1));
        assert_eq!(
            res.options,
            vec![&env, option]
        );
    }

    #[test]
    #[should_panic(expected = "Status(ContractError(6))")] // Error::OptionAlreadyExistsOnTrait
    fn duplicate_option_name() {
        let client = get_client();
        let env = &client.env;

        client.init(&bytes!(&env, 0xff), &10);
        // "a trait with options".hex => 612074726169742077697468206f7074696f6e73
        client.add_trait(&symbol!("has_opts"), &bytes!(env, 0x612074726169742077697468206f7074696f6e73));

        client.add_option(&symbol!("has_opts"), &symbol!("option_1"), &TraitOptionValue::Numeric(1));
        client.add_option(&symbol!("has_opts"), &symbol!("option_1"), &TraitOptionValue::Characters(bytes!(env, 0x00)));
    }

    #[test]
    fn draw() {
        let client = get_client();
        let env = &client.env;

        client.init(&bytes!(&env, 0xff), &10);
        // "a trait with options".hex => 612074726169742077697468206f7074696f6e73
        client.add_trait(&symbol!("trait_1"), &bytes!(env, 0x612074726169742077697468206f7074696f6e73));
        client.add_option(&symbol!("trait_1"), &symbol!("option_1"), &TraitOptionValue::Numeric(1));
        client.add_option(&symbol!("trait_1"), &symbol!("option_2"), &TraitOptionValue::Numeric(2));
        client.add_option(&symbol!("trait_1"), &symbol!("option_3"), &TraitOptionValue::Numeric(5));

        client.add_trait(&symbol!("trait_2"), &bytes!(env, 0x612074726169742077697468206f7074696f6e73));
        // "green".hex => 677265656e
        client.add_option(&symbol!("trait_2"), &symbol!("option_1"), &&TraitOptionValue::Characters(bytes!(&env, 0x677265656e)));
        // "red".hex => 726564
        client.add_option(&symbol!("trait_2"), &symbol!("option_2"), &&TraitOptionValue::Characters(bytes!(&env, 0x726564)));

        client.finalize();

        for _ in 0..10 {
            let random_bytes = <BytesN<32> as testutils::BytesN<32>>::random(&env) as BytesN<32>;
            _ = client.draw(&random_bytes)
        }
    }

    #[test]
    #[should_panic( expected = "Status(ContractError(8))" )]
    fn draw_over_limit() {
        let client = get_client();
        let env = &client.env;

        client.init(&bytes!(&env, 0xff), &1);
        // "a trait with options".hex => 612074726169742077697468206f7074696f6e73
        client.add_trait(&symbol!("trait_1"), &bytes!(env, 0x612074726169742077697468206f7074696f6e73));
        client.add_option(&symbol!("trait_1"), &symbol!("option_1"), &TraitOptionValue::Numeric(1));

        client.finalize();

        for _ in 0..=1 {
            let random_bytes = <BytesN<32> as testutils::BytesN<32>>::random(&env) as BytesN<32>;
            _ = client.draw(&random_bytes)
        }
    }

    #[test]
    fn draw_for_same_id() {
        let client = get_client();
        let env = &client.env;

        client.init(&bytes!(&env, 0xff), &2);
        // "a trait with options".hex => 612074726169742077697468206f7074696f6e73
        client.add_trait(&symbol!("trait_1"), &bytes!(env, 0x612074726169742077697468206f7074696f6e73));
        client.add_option(&symbol!("trait_1"), &symbol!("option_1"), &TraitOptionValue::Numeric(1));
        client.add_option(&symbol!("trait_1"), &symbol!("option_2"), &TraitOptionValue::Numeric(1));

        client.finalize();
        let draw_id = <BytesN<32> as testutils::BytesN<32>>::random(&env) as BytesN<32>;
        for _ in 0..5 {
            _ = client.draw(&draw_id)
        }

        let assigned_traits = env
            .as_contract(&client.contract_id, || env.storage().get_unchecked::<_, Map<BytesN<32>, Map<Symbol, TraitOptionValue>>>(ASSIGNED))
            .unwrap();

        assert_eq!(assigned_traits.len(), 1);
        assert!(assigned_traits.contains_key(draw_id));
    }
}
