use crate::{
    def::AttrVal,
    misc::EffectSpec,
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::UItemKey,
};

pub(crate) fn get_espec_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_espec: EffectSpec,
    projectee_key: UItemKey,
) -> Option<AttrVal> {
    let projector_r_effect = ctx.u_data.src.get_effect(projector_espec.effect_key);
    get_effect_proj_mult(ctx, calc, projector_espec.item_key, projector_r_effect, projectee_key)
}

pub(crate) fn get_effect_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: UItemKey,
) -> Option<AttrVal> {
    let prange = ctx.eff_projs.get_range(
        EffectSpec::new(projector_key, projector_effect.get_key()),
        projectee_key,
    )?;
    let proj_mult_getter = projector_effect.get_proj_mult_getter()?;
    Some(proj_mult_getter(ctx, calc, projector_key, projector_effect, prange))
}
