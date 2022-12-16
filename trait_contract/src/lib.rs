#![no_std]

use soroban_sdk::{contractimpl, panic_with_error, symbol, vec, Bytes, Env, Symbol, Vec};
use types::*;
use rand::Rng;
use soroban_rand::SorobanRng;

mod types;
mod tests;

pub const TRAITS: Symbol = symbol!("traits");
const COLLECTION: Symbol = symbol!("collection");
const FINAL: Symbol = symbol!("final");
const EMPTY: Symbol = symbol!("");

pub struct TraitContract;

#[contractimpl]
impl TraitContract {

    pub fn init(env: Env, name: Bytes, size: u32) -> Result<TraitCollection, Error>{
        if env.storage().has(COLLECTION) {
            panic_with_error!(&env, Error::AlreadyInitialized)
        }
        let collection: TraitCollection = TraitCollection{name: name, size: size};
        env.storage().set(COLLECTION, &collection);
        Ok(collection)
    }

    pub fn add_trait(env: Env, name: Symbol, desc: Bytes) -> Vec<AssetTrait> {
        let mut traits: Vec<AssetTrait> = Self::get_traits(env.clone());
        if traits.iter().any(|r| r.unwrap_or_default().name == name) {
            panic_with_error!(&env, Error::TraitExists)
        }
        traits.push_back(AssetTrait{name, desc, options: vec![&env]});
        env.storage().set(TRAITS, &traits);
        traits
    }

    pub fn add_option(env: Env, to_trait: Symbol, name: Symbol, value: TraitOptionValue) -> AssetTrait {
        assert!(name != EMPTY, "Must provide a option name");
        let e_trait = Self::get_trait(env.clone(), to_trait);
        if let Some(mut found) = e_trait {
            found.options.push_back(TraitOptionItem::new(name, Some(value)));
            Self::update_trait(env.clone(), &found);
            found
        } else {
            panic_with_error!(&env, Error::TraitNotFound);
        }
    }

    pub fn finalize(env: Env) -> bool {
        if let None = env.storage().get(COLLECTION) as Option<Result<TraitCollection, _>> {
            panic_with_error!(&env, Error::NotInitialized);
        }
        let collection_size = env.storage()
            .get_unchecked::<_, TraitCollection>(COLLECTION)
            .unwrap_or_default()
            .size;
        let traits = Self::get_traits(env.clone()).clone().iter();
        // all traits must have at least one option but not more options than collection size
        if traits.map(|r|r.unwrap_or_default()).any(|t| !t.check_is_ready(collection_size)) {
            panic_with_error!(&env, Error::TraitNotReady);
        }

        let mut updated_traits: Vec<AssetTrait> = vec![&env];
        for t in Self::get_traits(env.clone()).clone() {
            //let ut = t.unwrap();
            if let Some(ut) = t.unwrap().distribute_options(collection_size, env.clone(), Self::get_random_number) {
              updated_traits.push_back(ut.clone());
            } else {
                panic_with_error!(&env, Error::OptionDistributionFailed);
            }

        }
        env.storage().set(TRAITS, updated_traits);

        env.storage().set(FINAL, true);
        env.storage().get(FINAL).unwrap_or_else(|| Ok(false)).unwrap_or_default()
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

    fn update_trait(env: Env, updated: &AssetTrait) -> bool {
        let mut was_updated = false;
        let mut i = 0;
        let mut update_traits = Self::get_traits(env.clone());
        for _t in update_traits.iter() {
            let t = _t.unwrap().clone();
            if t.name == updated.name {
                update_traits.remove(i);
                update_traits.push_back(updated.clone());
                env.storage().set(TRAITS, update_traits);
                was_updated = true;
                break;
            }

            i += 1;
        }
        was_updated
    }

    fn get_random_number(e: &Env, min: u32, max: u32) -> u32 {
        let mut rng = SorobanRng::init(e.clone());
        rng.gen_range(min..=max)
    }
}
