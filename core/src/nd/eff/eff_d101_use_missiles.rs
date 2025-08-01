use crate::{
    ac, ec,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectHc},
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(ec::effects::USE_MISSILES),
        aid: ac::effects::USE_MISSILES,
        hc: NEffectHc {
            charge: Some(NEffectCharge {
                location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate {
                    can_run_uncharged: false,
                }),
                activates_charge: true,
            }),
            ..
        },
        ..
    }
}
