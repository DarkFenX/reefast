use crate::ed::{
    EAttr, EBuff, EDataCont, EEffect, EFighterAbil, EItem, EItemAttr, EItemEffect, EItemFighterAbil, EItemGroup,
    EItemList, EItemSkillReq, EItemSpaceComp, EMutaAttrMod, EMutaItemConv, EResult,
};

/// EVE data handler interface definition.
///
/// All the methods required by this trait should return an error only when it is impossible to
/// fetch the data altogether. In case of a less impactful error (such as inability to deserialize
/// one specific item within a big array of data), the error should be recorded as a meaningful
/// warning message and stored in [`EDataCont::warns`](self::EDataCont::warns).
pub trait EveDataHandler: std::fmt::Debug {
    /// Get item types.
    fn get_items(&self) -> EResult<EDataCont<EItem>>;
    /// Get item groups.
    fn get_item_groups(&self) -> EResult<EDataCont<EItemGroup>>;
    /// Get item type lists.
    fn get_item_lists(&self) -> EResult<EDataCont<EItemList>>;
    /// Get dogma attributes.
    fn get_attrs(&self) -> EResult<EDataCont<EAttr>>;
    /// Get an m:n mapping between item types and dogma attributes.
    fn get_item_attrs(&self) -> EResult<EDataCont<EItemAttr>>;
    /// Get dogma effects.
    fn get_effects(&self) -> EResult<EDataCont<EEffect>>;
    /// Get an m:n mapping between item types and dogma effects.
    fn get_item_effects(&self) -> EResult<EDataCont<EItemEffect>>;
    /// Get fighter abilities.
    fn get_fighter_abils(&self) -> EResult<EDataCont<EFighterAbil>>;
    /// Get an m:n mapping between item types and fighter abilities.
    fn get_item_fighter_abils(&self) -> EResult<EDataCont<EItemFighterAbil>>;
    /// Get dogma buffs.
    fn get_buffs(&self) -> EResult<EDataCont<EBuff>>;
    /// Get space components.
    fn get_space_comps(&self) -> EResult<EDataCont<EItemSpaceComp>>;
    /// Get item skill requirements.
    fn get_item_skill_reqs(&self) -> EResult<EDataCont<EItemSkillReq>>;
    /// Get mutator item conversions.
    fn get_muta_item_convs(&self) -> EResult<EDataCont<EMutaItemConv>>;
    /// Get mutator item modifications.
    fn get_muta_attr_mods(&self) -> EResult<EDataCont<EMutaAttrMod>>;
    /// Get version of the data.
    fn get_data_version(&self) -> EResult<String>;
}
