use crate::{
    ac, ad, ec, ed,
    nd::{
        NEffect, NEffectHc,
        eff::shared::{
            damp::update_effect,
            proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
        },
    },
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::REMOTE_SENSOR_DAMP_ENTITY;
const A_EFFECT_ID: ad::AEffectId = ac::effects::REMOTE_SENSOR_DAMP_ENTITY;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(|a_effect| update_effect(A_EFFECT_ID, a_effect)),
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        hc: NEffectHc {
            proj_mult_getter: Some(get_proj_mult_simple_s2s),
            ..
        },
        ..
    }
}
