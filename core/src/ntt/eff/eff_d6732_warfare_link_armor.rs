use crate::{ac, ad, ec, ntt::NttEffect};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::MOD_BONUS_WARFARE_LINK_ARMOR),
        aid: ac::effects::MOD_BONUS_WARFARE_LINK_ARMOR,
        adg_buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::DefaultAttrs,
            scope: ad::AEffectBuffScope::FleetShips,
        }),
        adg_charge_info: Some(ad::AEffectChargeInfo::Loaded),
        ..
    }
}
