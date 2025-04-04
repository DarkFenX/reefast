use crate::util::RMap;
pub use attr::AAttr;
pub use buff::{ABuff, ABuffAffecteeFilter, ABuffAggrMode, ABuffModifier};
pub use effect::{
    AEffect, AEffectAffecteeFilter, AEffectBuffInfo, AEffectBuffScope, AEffectBuffSrc, AEffectBuffSrcCustom,
    AEffectChargeInfo, AEffectId, AEffectLocation, AEffectModBuildStatus, AEffectModifier,
};
pub use item::{
    AItem, AItemChargeLimit, AItemEffectData, AItemExtras, AItemKind, AItemShipLimit, AShipDroneLimit, AShipKind,
};
pub use muta::{AMuta, AMutaAttrRange};
pub use primitives::{
    AAttrId, AAttrVal, ABuffId, ACount, ACustomEffectId, ADogmaEffectId, AEffectCatId, AItemCatId, AItemGrpId, AItemId,
    ASkillLevel, ASlotIndex, ArcAttr, ArcBuff, ArcEffect, ArcItem, ArcMuta,
};
pub use shared::{AModifierSrq, AOp, AState};

mod attr;
mod buff;
mod effect;
mod item;
mod muta;
mod primitives;
mod shared;

/// Adapted data storage.
pub struct AData {
    pub items: RMap<AItemId, AItem>,
    pub attrs: RMap<AAttrId, AAttr>,
    pub mutas: RMap<AItemId, AMuta>,
    pub effects: RMap<AEffectId, AEffect>,
    pub buffs: RMap<ABuffId, ABuff>,
}
impl AData {
    pub(crate) fn new() -> Self {
        Self {
            items: RMap::new(),
            attrs: RMap::new(),
            mutas: RMap::new(),
            effects: RMap::new(),
            buffs: RMap::new(),
        }
    }
}
