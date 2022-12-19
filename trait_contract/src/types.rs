use soroban_sdk::{bytes, contracterror, contracttype, symbol, vec, Bytes, Env, Symbol, Vec};
use crate::get_random_number;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 0,
    AlreadyInitialized = 1,
    TraitExists = 2,
    TraitNotFound = 3,
    TraitNotReady = 4,
    OptionDistributionFailed = 5,
    OptionAlreadyExistsOnTrait = 6,
    NotFinalized = 7,
    NoTraitsLeft = 8,
    OptionExhausted = 9,
}

#[contracttype]
#[derive(Debug, PartialEq)]
pub struct TraitCollection {
    pub name: Bytes,
    pub size: u32,
}

impl Default for TraitCollection {
    fn default() -> Self {
        TraitCollection { name: Bytes::from_array(&Env::default(), &[]), size: 0 }
    }
}

#[contracttype]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub enum TraitOptionValue {
    #[default]
    None,
    Numeric(u32),
    Characters(Bytes),
}

#[contracttype]
#[derive(Debug, Clone)]
pub struct TraitOptionItem {
    pub name: Symbol,
    pub value: TraitOptionValue,
    pub available: u32,
    pub total: u32,
}

impl TraitOptionItem {
    pub fn new(name: Symbol, value: Option<TraitOptionValue>) -> Self {
        Self{
            name,
            value: value.unwrap_or_default(),
            available: u32::default(),
            total: u32::default(),
        }
    }

    pub fn with_distribution(self: Self, distribution: u32) -> Self {
        let mut new_self = self.clone();
        new_self.available = distribution;
        new_self.total = distribution;
        new_self
    }

    pub fn with_decremented_available(self: Self) -> Result<Self, Error> {
        let mut new_self = self.clone();
        if new_self.available > 0 {
            new_self.available -= 1;
            return Ok(new_self);
        }
        return Err(Error::OptionExhausted)
    }

}

#[contracttype]
#[derive(Debug, Clone, PartialEq)]
pub struct AssetTrait {
    pub name: Symbol,
    pub desc: Bytes,
    pub options: Vec<TraitOptionItem>,
}

impl Default for AssetTrait {
    fn default() -> Self {
        AssetTrait {
            name: symbol!(""),
            desc: bytes!(&Env::default()),
            options: vec![&Env::default()]
        }
    }
}

impl AssetTrait {
    pub fn check_is_ready(self: &Self, max_options: u32) -> bool {
        if self.name == symbol!("") {
            return false;
        }
        if self.options.len() == 0 || self.options.len() > max_options {
            return false;
        }
        true
    }

    pub fn distribute_options(self: Self, total_options: u32, env: Env) -> Option<AssetTrait> {
        let mut unassigned_items = self.options.len();
        let mut assigned_options = 0;
        let mut res = self.clone();

        while unassigned_items > 0 {
            unassigned_items -= 1;

            let distribution: u32;
            if unassigned_items > 0 {
                distribution = get_random_number(&env, 1, total_options - assigned_options - unassigned_items);
            } else {
                distribution = total_options - assigned_options;
            }
            assigned_options += distribution;

            let new_option = self.options
                .get_unchecked(unassigned_items)
                .unwrap()
                .with_distribution(distribution);
            res.options.set(unassigned_items, new_option);
        }

        if assigned_options == total_options {
            return Some(res);
        }
        None
    }

    /// gets vector-indices of available options weighted by total options
    ///
    /// as long as there are options available the original probability of being randomly
    /// picked stays intact
    /// only exhausted options are skipped.
    ///
    /// # example
    /// AssetTrait{ options: [
    ///   TraitOptionItem{ total: 10, available: 0},
    ///   TraitOptionItem{ total: 3, available: 2},
    ///   TraitOptionItem{ total: 5, available: 1},
    /// ]}.get_available_options -> [1, 1, 1, 2, 2, 2, 2, 2]
    pub fn get_available_options(self: Self, env: Env) -> Vec<u32> {
        let mut opts_map: Vec<u32> = vec![&env];
        for i in 0..self.options.len() {
            if let Ok(option) = self.options.get_unchecked(i) {
                if option.available == 0 {
                    continue;
                }
                for _ in 0..option.total {
                    opts_map.push_back(i)
                }
            }
        }
        opts_map
    }
}
