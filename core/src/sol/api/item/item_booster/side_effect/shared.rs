use crate::{
    ad::{AAttrId, AEffectId},
    rd::REffectKey,
    src::Src,
};

pub(super) fn get_se_chance_attr_id_by_effect_id(src: &Src, effect_id: &AEffectId) -> Option<AAttrId> {
    let effect_key = src.get_effect_key_by_id(effect_id)?;
    get_se_chance_attr_id_by_effect_key(src, effect_key)
}

pub(super) fn get_se_chance_attr_id_by_effect_key(src: &Src, effect_key: REffectKey) -> Option<AAttrId> {
    src.get_effect(effect_key).get_chance_attr_id()
}
