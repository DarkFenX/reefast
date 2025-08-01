use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::MINING_LASER;
const A_EFFECT_ID: AEffectId = ac::effects::MINING_LASER;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        hc: NEffectHc {
            charge: Some(NEffectCharge {
                location: NEffectChargeLoc::Loaded(NEffectChargeDepl::Crystal {
                    can_run_uncharged: true,
                }),
                activates_charge: false,
            }),
            proj_mult_getter: Some(get_proj_mult_simple_s2s),
            ..
        },
        ..
    }
}
