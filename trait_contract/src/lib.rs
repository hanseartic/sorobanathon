#![no_std]

use soroban_sdk::{contractimpl, map, panic_with_error, symbol, vec, Bytes, BytesN, Env, Map, Symbol, Vec};
use types::*;
use rand::Rng;
use soroban_rand::SorobanRng;

mod types;
mod tests;

pub const TRAITS: Symbol = symbol!("traits");
pub const ASSIGNED: Symbol = symbol!("assigned");
const COLLECTION: Symbol = symbol!("collection");
const IS_FINAL: Symbol = symbol!("final");
const EMPTY: Symbol = symbol!("");

pub fn get_random_number(e: &Env, min: u32, max: u32) -> u32 {
    let mut rng = SorobanRng::init(e.clone());
    rng.gen_range(min..=max)
}

pub struct TraitContract;

#[contractimpl]
impl TraitContract {

    /// Initializes the contract with a name and desired collection size.
    ///
    /// The name is not evaluated at the moment. Could be used to allow the contract to
    /// manage multiple collections.
    /// The size determines how many trait-sets can be drawn from the contract.
    ///
    /// Returns the created [`TraitCollection`] as confirmation.
    pub fn init(env: Env, name: Bytes, size: u32) -> Result<TraitCollection, Error>{
        if env.storage().has(COLLECTION) {
            panic_with_error!(&env, Error::AlreadyInitialized)
        }
        let collection: TraitCollection = TraitCollection{name: name, size: size};
        env.storage().set(COLLECTION, &collection);
        Ok(collection)
    }

    /// Add a trait to the collection.
    ///
    /// Traits are the holdings options for a certain property to be assigned.
    /// For these properties could be numeric values (e.g. strength, age, ...) represented as [`u32`] or
    /// any property of the image (e.g. 'background color', 'accessoire', ...) represented as [`Bytes`].
    ///
    /// Returns all traits that have been added to the contract.
    ///
    /// Returns a [`Vec`] of all currently existing [`AssetTrait`] of the contract.
    pub fn add_trait(env: Env, name: Symbol, desc: Bytes) -> Vec<AssetTrait> {
        Self::expect_initialized(env.clone());
        let mut traits: Vec<AssetTrait> = Self::get_traits(env.clone());
        if traits.iter().any(|r| r.unwrap_or_default().name == name) {
            panic_with_error!(&env, Error::TraitExists)
        }
        traits.push_back(AssetTrait{name, desc, options: vec![&env]});
        env.storage().set(TRAITS, &traits);
        traits
    }

    /// Add an option to a trait.
    ///
    /// Add a [`TraitOptionItem`] to an [`AssetTrait`] identified by name.
    ///
    /// Returns the trait with all currently added options.
    ///
    /// Return [`AssetTrait`]
    pub fn add_option(env: Env, to_trait: Symbol, option_name: Symbol, option_value: TraitOptionValue) -> AssetTrait {
        Self::expect_initialized(env.clone());
        assert!(option_name != EMPTY, "Must provide an option name");
        if let Some(mut found) = Self::get_trait(env.clone(), to_trait) {
            if Self::trait_has_option(found.clone(), option_name) {
                panic_with_error!(&env, Error::OptionAlreadyExistsOnTrait);
            }
            found.options.push_back(TraitOptionItem::new(option_name, Some(option_value)));
            Self::update_trait(env.clone(), found.clone());
            found
        } else {
            panic_with_error!(&env, Error::TraitNotFound);
        }
    }

    /// Finalize the contract
    ///
    /// Finalizing a contract will prevent adding more traits or options. Furthermore it will distribute the options
    /// over the size of the collection.
    ///
    /// Given a [`TraitCollection`].size of 10 and a [`AssetTrait`] with three [`TraitOptionItem`],
    /// the options for the trait will be randomly distributed over the collection.
    ///
    /// E.g. defining a 'background-color' trait with three options (red, green, blue) could end up distributed like this:
    /// ```
    /// AssetTrait {
    ///   name: "background",
    ///   desc: 0x4261636b67726f756e6420636f6c6f72, // <= "Backgroun color"
    ///   options: [
    ///     TraitOptionItem {
    ///       name: "red",
    ///       value: TraitOptionValue::Charatcer("red"),
    ///       available: 3,
    ///       initial: 3,
    ///     },
    ///     TraitOptionItem {
    ///       name: "green",
    ///       value: TraitOptionValue::Charatcer("green"),
    ///       available: 2,
    ///       initial: 2,
    ///     },
    ///     TraitOptionItem {
    ///       name: "blue",
    ///       value: TraitOptionValue::Charatcer("blue"),
    ///       available: 5,
    ///       initial: 5,
    ///     }
    ///   ],
    /// }
    /// ```
    pub fn finalize(env: Env) -> bool {
        Self::expect_initialized(env.clone());
        let collection_size = env.storage()
            .get_unchecked::<_, TraitCollection>(COLLECTION)
            .unwrap_or_default()
            .size;
        let traits = Self::get_traits(env.clone()).clone().iter();
        // all traits must have at least one option but not more options than collection size
        if traits.map(|r|r.unwrap_or_default()).any(|t| !t.check_is_ready(collection_size)) {
            panic_with_error!(&env, Error::TraitNotReady);
        }

        let mut asset_traits = Self::get_traits(env.clone());
        for i in 0..asset_traits.len() {
            if let Some(ut) = asset_traits.get_unchecked(i).unwrap().distribute_options(collection_size, env.clone()) {
                asset_traits.set(i, ut);
            } else {
                panic_with_error!(&env, Error::OptionDistributionFailed);
            }
        }
        env.storage().set(TRAITS, asset_traits);

        env.storage().set(ASSIGNED, map!(&env) as Map<BytesN<32>, Map<Symbol, TraitOptionValue>>);

        env.storage().set(IS_FINAL, true);
        env.storage().get(IS_FINAL).unwrap_or_else(|| Ok(false)).unwrap_or_default()
    }

    /// Draw an option (trait-set) from the pool.
    ///
    /// Provide an identifier to draw the trait-set for (could be sha256 of an asset descriptor
    /// e.g. "RUSTLING:GARCMLC7PX4H47NWANR3TBY52OJCBWZ6N54IIIIT32GX3OD7J5OXU6HQ" => 0d6d7e76e304748ee0310f6a1fa95eda9d49d9dcf2a6b1d64582413b9702f891)
    ///
    /// Notes:
    /// - subsequent invokes with the same id will return the already assigned option => there is only one option per ID
    /// - after <collection size>-amount of results have been drawn the collection is exhausted and subsequent invokes (except for
    ///    already known IDs, see above) will fail.
    pub fn draw(env: Env, id: BytesN<32>) -> Result<Map<Symbol, TraitOptionValue>, Error> {
        Self::expect_finalized(env.clone());

        let mut assigned_traits = env.storage()
            .get_unchecked::<_, Map<BytesN<32>, Map<Symbol, TraitOptionValue>>>(ASSIGNED).unwrap();

        if assigned_traits.len() == env.storage().get_unchecked::<_, TraitCollection>(COLLECTION)
            .unwrap().size {
                panic_with_error!(&env, Error::NoTraitsLeft)
        }

        let mut selected_options = assigned_traits.get(id.clone())
            .unwrap_or_else(||Ok(map![&env])).unwrap();

        if selected_options.len() > 0 {
            return Ok(selected_options);
        }

        let asset_traits = Self::get_traits(env.clone());
        for i in 0..asset_traits.len() {
            if let Some(Ok(current_trait)) = asset_traits.get(i) {
                let option_indexes = current_trait.clone().get_available_options(env.clone());
                let selected_index = option_indexes.get_unchecked(get_random_number(&env, 0, option_indexes.len()-1)).unwrap();
                if let Some(Ok(option)) = current_trait.options.get(selected_index) {
                    selected_options.set(current_trait.name, option.value);
                }
            }
        }
        assigned_traits.set(id, selected_options.clone());
        env.storage().set(ASSIGNED, assigned_traits);

        // todo: figure a way to identify trait-set
        // for now just use input
        Ok(selected_options)
    }

    fn expect_initialized(env: Env) {
        if !env.storage().has(COLLECTION) {
            panic_with_error!(&env, Error::NotInitialized)
        }
    }

    fn expect_finalized(env: Env) {
        if !env.storage().get::<_, bool>(IS_FINAL).unwrap_or_else(||Ok(false)).unwrap_or_else(|_|false) {
            panic_with_error!(&env, Error::NotFinalized)
        }
    }

    fn get_trait(env: Env, name: Symbol) -> Option<AssetTrait> {
        assert!(name != EMPTY, "Must provide a trait name");
        let mut traits = Self::get_traits(env.clone())
            .into_iter()
            .map(|r|r.unwrap_or_default());
        if let Some(found) = traits.find(|t| t.name == name) {
            Some(found)
        } else {
            None
        }
    }

    fn get_traits(env: Env) -> Vec<AssetTrait> {
        env.storage()
            .get(TRAITS)
            .unwrap_or_else(|| Ok(vec![&env]))
            .unwrap()
    }

    fn update_trait(env: Env, updated: AssetTrait) -> bool {
        let mut was_updated = false;
        let mut update_traits = Self::get_traits(env.clone());

        for i in 0..update_traits.len() {
            let t = update_traits.get_unchecked(i).unwrap();
            if t.name == updated.name {
                update_traits.set(i, updated);
                env.storage().set(TRAITS, update_traits);
                was_updated = true;
                break;
            }
        }
        was_updated
    }

    fn trait_has_option(t: AssetTrait, option_name: Symbol) -> bool {
        let mut has_option = false;
        if let Some(option_exists) = t.options.iter().find(|o|o.is_ok() && o.as_ref().unwrap().name == option_name) {
            has_option = option_exists.is_ok();
        }
        has_option
    }
}
