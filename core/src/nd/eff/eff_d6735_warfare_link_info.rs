use crate::{
    ac, ad, ec,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectHc},
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(ec::effects::MOD_BONUS_WARFARE_LINK_INFO),
        aid: ac::effects::MOD_BONUS_WARFARE_LINK_INFO,
        adg_buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::DefaultAttrs,
            scope: ad::AEffectBuffScope::FleetShips,
        }),
        hc: NEffectHc {
            charge: Some(NEffectCharge {
                location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate {
                    can_run_uncharged: false,
                }),
                activates_charge: false,
            }),
            ..
        },
        ..
    }
}
