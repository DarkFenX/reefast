use std::error;
use std::fmt;
use std::result;

use super::data::{
    Attr, Buff, Container, Effect, FighterAbil, Item, ItemAttr, ItemEffect, ItemFighterAbil, ItemGroup, ItemSkillReq,
    MutaAttrMod, MutaItemConv,
};

/// Alias for a `Result` which accepts any error type
pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

/// Data handler interface definition.
pub trait DataHandler: fmt::Debug {
    /// Get item types.
    fn get_items(&self) -> Result<Container<Item>>;
    /// Get item groups.
    fn get_item_groups(&self) -> Result<Container<ItemGroup>>;
    /// Get dogma attributes.
    fn get_attrs(&self) -> Result<Container<Attr>>;
    /// Get an m:n mapping between item types and dogma attributes.
    fn get_item_attrs(&self) -> Result<Container<ItemAttr>>;
    /// Get dogma effects.
    fn get_effects(&self) -> Result<Container<Effect>>;
    /// Get an m:n mapping between item types and dogma effects.
    fn get_item_effects(&self) -> Result<Container<ItemEffect>>;
    /// Get fighter abilities.
    fn get_fighter_abils(&self) -> Result<Container<FighterAbil>>;
    /// Get an m:n mapping between item types and fighter abilities.
    fn get_item_fighter_abils(&self) -> Result<Container<ItemFighterAbil>>;
    /// Get dogma buffs.
    fn get_buffs(&self) -> Result<Container<Buff>>;
    /// Get item skill requirements.
    fn get_item_skill_reqs(&self) -> Result<Container<ItemSkillReq>>;
    /// Get mutaplasmid item conversions.
    fn get_muta_item_convs(&self) -> Result<Container<MutaItemConv>>;
    /// Get mutaplasmid item modifications.
    fn get_muta_attr_mods(&self) -> Result<Container<MutaAttrMod>>;
    /// Get version of the data.
    fn get_version(&self) -> Result<String>;
}
