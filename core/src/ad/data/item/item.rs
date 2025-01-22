use crate::{
    ad::{AItemEffectData, AItemExtras},
    defs::{AttrVal, EAttrId, EEffectId, EItemCatId, EItemGrpId, EItemId, SkillLevel},
    util::{Named, StMap},
};

/// Represents an adapted item.
///
/// An item carries alot of info needed to calculate fit attributes, for example base attribute
/// values.
pub struct AItem {
    /// Item ID.
    pub id: EItemId,
    /// Item group ID.
    pub grp_id: EItemGrpId,
    /// Item category ID.
    pub cat_id: EItemCatId,
    /// Attribute values of the item.
    pub attrs: StMap<EAttrId, AttrVal>,
    /// Refers effects of the item.
    pub effect_datas: StMap<EEffectId, AItemEffectData>,
    /// Refers an effect which is default for the item.
    pub defeff_id: Option<EEffectId>,
    /// Skill requirement map.
    pub srqs: StMap<EItemId, SkillLevel>,
    /// Struct with extra data which is calculated during cache generation.
    pub extras: AItemExtras,
}
impl AItem {
    /// Make a new adapted item type out of passed data.
    pub(crate) fn new(
        id: EItemId,
        grp_id: EItemGrpId,
        cat_id: EItemCatId,
        attr_vals: StMap<EAttrId, AttrVal>,
        effect_datas: StMap<EEffectId, AItemEffectData>,
        defeff_id: Option<EEffectId>,
        srqs: StMap<EItemId, SkillLevel>,
    ) -> Self {
        Self {
            id,
            grp_id,
            cat_id,
            attrs: attr_vals,
            effect_datas,
            defeff_id,
            srqs,
            extras: AItemExtras::new(),
        }
    }
}
impl Named for AItem {
    fn get_name() -> &'static str {
        "AItem"
    }
}
impl std::fmt::Display for AItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={})", Self::get_name(), self.id)
    }
}
