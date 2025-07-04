use crate::{
    ac, ad, ec, ed,
    ntt::{NttEffect, eff::shared::missile_dmg_self_srq::update_effect},
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::MISSILE_THERM_DMG_BONUS;
const A_EFFECT_ID: ad::AEffectId = ac::effects::MISSILE_THERM_DMG_BONUS;

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_custom_fn: Some(|a_data| update_effect(a_data, A_EFFECT_ID, ac::attrs::THERM_DMG)),
        ..
    }
}
