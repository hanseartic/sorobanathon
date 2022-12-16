use soroban_sdk::{bytes, contracterror, contracttype, symbol, vec, Bytes, Env, Symbol, Vec};


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
}

impl TraitOptionItem {
    pub fn new(name: Symbol, value: Option<TraitOptionValue>) -> Self {
        Self{
            name,
            value: value.unwrap_or_default(),
            available: u32::default(),
        }
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

    pub fn distribute_options(self: Self, total_options: u32, env: Env, rand: fn(env: &Env, u32, u32) -> u32) -> Option<AssetTrait> {
        let mut unassigned_items = self.options.len();
        let mut assigned_options = 0;
        let mut res = self.clone();

        while unassigned_items > 0 {
            unassigned_items -= 1;

            let mut o = self.options.get(unassigned_items).unwrap().unwrap();

            if unassigned_items > 0 {
                o.available = rand(&env, 1, total_options - assigned_options - unassigned_items);
            } else {
                o.available = total_options - assigned_options;
            }
            assigned_options += o.available;

            res.options.set(unassigned_items, o);
        }

        if assigned_options == total_options {
            return Some(res);
        }
        None
    }
}
