use crate::{
    ac,
    ad::{
        AEffect, AEffectBuffInfo, AEffectBuffScope, AEffectBuffSrc, AEffectBuffSrcCustom, AEffectId, AItemId, AState,
    },
    def::OF,
    nd::{NEffect, eff::shared::sov_stability_generators::assign_effect},
};

const A_ITEM_ID: AItemId = ac::items::ELECTRIC_STABILITY_GENERATOR;
const A_EFFECT_ID: AEffectId = ac::effects::STABILITY_GENERATOR_ELECTRIC;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: A_EFFECT_ID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(|a_items| assign_effect(a_items, A_ITEM_ID, A_EFFECT_ID)),
        ..
    }
}

fn make_effect() -> AEffect {
    AEffect {
        id: A_EFFECT_ID,
        category: ac::effcats::ACTIVE,
        state: AState::Active,
        buff_info: Some(AEffectBuffInfo {
            source: AEffectBuffSrc::Customized(vec![
                AEffectBuffSrcCustom::HardcodedVal(ac::buffs::SOV_SMOD_CAPACITOR_RECHARGE_BONUS, OF(-25.0)),
                AEffectBuffSrcCustom::HardcodedVal(ac::buffs::SOV_SMOD_TARGETING_AND_DSCAN_RANGE_BONUS, OF(25.0)),
            ]),
            scope: AEffectBuffScope::Ships,
        }),
        ..
    }
}
