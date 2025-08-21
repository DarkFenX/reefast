use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        eff::shared::{
            damp::update_effect,
            proj_mult::{get_full_mod_proj_attrs, get_noapp_full_proj_mult},
        },
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::REMOTE_SENSOR_DAMP_FALLOFF;
const A_EFFECT_ID: AEffectId = ac::effects::REMOTE_SENSOR_DAMP_FALLOFF;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(|a_effect| update_effect(A_EFFECT_ID, a_effect)),
        modifier_proj_attrs_getter: Some(get_full_mod_proj_attrs),
        hc: NEffectHc {
            modifier_proj_mult_getter: Some(get_noapp_full_proj_mult),
            ..
        },
        ..
    }
}
