use crate::{
    ac, ad,
    def::{AttrVal, Count, ItemKey, OF},
    ec,
    misc::{DmgKinds, Spool},
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
    },
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputComplex, OutputSimple},
    },
    util::floor_unerr,
};

pub(super) fn mk_n_effect() -> NEffect {
    // Dreadnought lance
    NEffect {
        eid: Some(ec::effects::DEBUFF_LANCE),
        aid: ac::effects::DEBUFF_LANCE,
        adg_buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::Customized(vec![
                ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::REMOTE_REPAIR_IMPEDANCE, OF(-50.0)),
                ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::WARP_PENALTY, OF(100.0)),
                ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::DISALLOW_DOCK_JUMP, OF(1.0)),
                ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::DISALLOW_TETHER, OF(1.0)),
            ]),
            scope: ad::AEffectBuffScope::Everything,
        }),
        // TODO: test if it uses surface-to-surface range (might use center-to-surface), and check
        // TODO: if damage radius is needed to be added to range or not
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        hc: NEffectHc {
            get_proj_mult: Some(get_proj_mult_simple_s2s),
            get_normal_dmg_opc: Some(get_dmg_opc),
            ..
        },
        ..
    }
}

fn get_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: ItemKey,
    _projector_a_effect: &ad::AEffectRt,
    _spool: Option<Spool>,
    _projectee_key: Option<ItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    let dmg_em = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::EM_DMG)?;
    let dmg_therm = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::THERM_DMG)?;
    let dmg_kin = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::KIN_DMG)?;
    let dmg_expl = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::EXPL_DMG)?;
    let delay =
        calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DOOMSDAY_WARNING_DURATION)? / OF(1000.0);
    Some(
        match calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DOOMSDAY_DAMAGE_CYCLE_TIME)? {
            interval if interval > OF(0.0) => {
                let duration =
                    calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DOOMSDAY_DAMAGE_DURATION)?
                        / OF(1000.0);
                let repeats = floor_unerr(duration / interval) as Count;
                Output::Complex(OutputComplex {
                    amount: DmgKinds {
                        em: dmg_em,
                        thermal: dmg_therm,
                        kinetic: dmg_kin,
                        explosive: dmg_expl,
                    },
                    delay,
                    repeats,
                    interval,
                })
            }
            _ => Output::Simple(OutputSimple {
                amount: DmgKinds {
                    em: dmg_em,
                    thermal: dmg_therm,
                    kinetic: dmg_kin,
                    explosive: dmg_expl,
                },
                delay,
            }),
        },
    )
}
