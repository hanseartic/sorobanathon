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
}
