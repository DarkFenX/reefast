use crate::{
    ac,
    ad::AEffectId,
    def::AttrVal,
    ec,
    ed::EEffectId,
    misc::{ResolvedSpool, Spool},
    nd::{
        NEffect, NEffectHc,
        eff::shared::{
            proj_mult::get_noapp_simple_s2s_proj_mult, rep_opc::get_remote_armor_rep_opc, spool::get_resolved_spool,
        },
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::SHIP_MOD_REMOTE_ARMOR_MUTADAPTIVE_REPAIRER;
const A_EFFECT_ID: AEffectId = ac::effects::SHIP_MOD_REMOTE_ARMOR_MUTADAPTIVE_REPAIRER;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            spool_resolver: Some(internal_get_resolved_spool),
            remote_armor_rep_opc_getter: Some(internal_get_remote_rep_opc),
            ..
        },
        ..
    }
}

fn internal_get_resolved_spool(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    r_effect: &REffect,
    spool: Option<Spool>,
) -> Option<ResolvedSpool> {
    get_resolved_spool(
        ctx,
        calc,
        item_key,
        r_effect,
        spool,
        &ac::attrs::REP_MULT_BONUS_PER_CYCLE,
        &ac::attrs::REP_MULT_BONUS_MAX,
    )
}

fn internal_get_remote_rep_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<Output<AttrVal>> {
    get_remote_armor_rep_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        spool,
        Some(internal_get_resolved_spool),
        projectee_key,
        get_noapp_simple_s2s_proj_mult,
    )
}
