use crate::sol::{
    FitId,
    svc::calc::{AffecteeFilter, CtxModifier, RawModifier, registers::StandardRegister},
    uad::Uad,
};

use super::{add_ctx_modifier, remove_ctx_modifier};

impl StandardRegister {
    pub(in crate::sol::svc::calc) fn reg_sw_system_mod(
        &mut self,
        ctx_modifiers: &mut Vec<CtxModifier>,
        uad: &Uad,
        raw_modifier: RawModifier,
    ) {
        ctx_modifiers.clear();
        let valid = match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    ctx_modifiers.reserve(uad.fits.len());
                    for &fit_id in uad.fits.iter_fit_ids() {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                        add_ctx_modifier(
                            &mut self.cmods_root,
                            (fit_id, loc_kind),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                    true
                }
                _ => false,
            },
            AffecteeFilter::Loc(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    ctx_modifiers.reserve(uad.fits.len());
                    for &fit_id in uad.fits.iter_fit_ids() {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                        add_ctx_modifier(
                            &mut self.cmods_loc,
                            (fit_id, loc_kind),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                    true
                }
                _ => false,
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    ctx_modifiers.reserve(uad.fits.len());
                    for &fit_id in uad.fits.iter_fit_ids() {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                        add_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            (fit_id, loc_kind, a_item_grp_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                    true
                }
                _ => false,
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    ctx_modifiers.reserve(uad.fits.len());
                    for &fit_id in uad.fits.iter_fit_ids() {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                        add_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            (fit_id, loc_kind, srq_a_item_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                    true
                }
                _ => false,
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => {
                ctx_modifiers.reserve(uad.fits.len());
                for &fit_id in uad.fits.iter_fit_ids() {
                    let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                    add_ctx_modifier(
                        &mut self.cmods_own_srq,
                        (fit_id, srq_a_item_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    ctx_modifiers.push(ctx_modifier);
                }
                true
            }
        };
        if valid {
            self.rmods_sw_system.insert(raw_modifier);
            self.rmods_nonproj
                .add_entry((raw_modifier.affector_item_key, raw_modifier.a_effect_id), raw_modifier);
        }
    }
    pub(in crate::sol::svc::calc) fn unreg_sw_system_mod(
        &mut self,
        ctx_modifiers: &mut Vec<CtxModifier>,
        uad: &Uad,
        raw_modifier: RawModifier,
    ) {
        ctx_modifiers.clear();
        match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => {
                if let Ok(loc_kind) = loc.try_into() {
                    ctx_modifiers.reserve(uad.fits.len());
                    for &fit_id in uad.fits.iter_fit_ids() {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                        remove_ctx_modifier(
                            &mut self.cmods_root,
                            &(fit_id, loc_kind),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
            }
            AffecteeFilter::Loc(loc) => {
                if let Ok(loc_kind) = loc.try_into() {
                    ctx_modifiers.reserve(uad.fits.len());
                    for &fit_id in uad.fits.iter_fit_ids() {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                        remove_ctx_modifier(
                            &mut self.cmods_loc,
                            &(fit_id, loc_kind),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
            }
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => {
                if let Ok(loc_kind) = loc.try_into() {
                    ctx_modifiers.reserve(uad.fits.len());
                    for &fit_id in uad.fits.iter_fit_ids() {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            &(fit_id, loc_kind, a_item_grp_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
            }
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => {
                if let Ok(loc_kind) = loc.try_into() {
                    ctx_modifiers.reserve(uad.fits.len());
                    for &fit_id in uad.fits.iter_fit_ids() {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            &(fit_id, loc_kind, srq_a_item_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
            }
            AffecteeFilter::OwnSrq(srq_a_item_id) => {
                ctx_modifiers.reserve(uad.fits.len());
                for &fit_id in uad.fits.iter_fit_ids() {
                    let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                    remove_ctx_modifier(
                        &mut self.cmods_own_srq,
                        &(fit_id, srq_a_item_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    ctx_modifiers.push(ctx_modifier);
                }
            }
        }
        self.rmods_sw_system.remove(&raw_modifier);
    }
    // No need to return any ctx modifiers here, since fits being added have no items
    pub(in crate::sol::svc::calc) fn reg_fit_for_sw(&mut self, fit_id: FitId) {
        for raw_modifier in self.rmods_sw_system.iter() {
            match raw_modifier.affectee_filter {
                AffecteeFilter::Direct(loc) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(*raw_modifier, fit_id);
                        add_ctx_modifier(
                            &mut self.cmods_root,
                            (fit_id, loc_kind),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::Loc(loc) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(*raw_modifier, fit_id);
                        add_ctx_modifier(
                            &mut self.cmods_loc,
                            (fit_id, loc_kind),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::LocGrp(loc, a_item_grp_id) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(*raw_modifier, fit_id);
                        add_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            (fit_id, loc_kind, a_item_grp_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::LocSrq(loc, srq_a_item_id) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(*raw_modifier, fit_id);
                        add_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            (fit_id, loc_kind, srq_a_item_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::OwnSrq(srq_a_item_id) => {
                    let ctx_modifier = CtxModifier::from_raw_with_fit(*raw_modifier, fit_id);
                    add_ctx_modifier(
                        &mut self.cmods_own_srq,
                        (fit_id, srq_a_item_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                }
            }
        }
    }
    // No need to return any ctx modifiers here, since fits being removed have no items
    pub(in crate::sol::svc::calc) fn unreg_fit_for_sw(&mut self, fit_id: FitId) {
        for raw_modifier in self.rmods_sw_system.iter() {
            match raw_modifier.affectee_filter {
                AffecteeFilter::Direct(loc) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(*raw_modifier, fit_id);
                        remove_ctx_modifier(
                            &mut self.cmods_root,
                            &(fit_id, loc_kind),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::Loc(loc) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(*raw_modifier, fit_id);
                        remove_ctx_modifier(
                            &mut self.cmods_loc,
                            &(fit_id, loc_kind),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::LocGrp(loc, a_item_grp_id) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(*raw_modifier, fit_id);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            &(fit_id, loc_kind, a_item_grp_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::LocSrq(loc, srq_a_item_id) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(*raw_modifier, fit_id);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            &(fit_id, loc_kind, srq_a_item_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::OwnSrq(srq_a_item_id) => {
                    let ctx_modifier = CtxModifier::from_raw_with_fit(*raw_modifier, fit_id);
                    remove_ctx_modifier(
                        &mut self.cmods_own_srq,
                        &(fit_id, srq_a_item_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                }
            }
        }
    }
}
