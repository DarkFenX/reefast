use std::convert::TryInto;

use crate::sol::{
    item::{SolFwEffect, SolItem, SolShipKind},
    svc::svce_calc::{
        modifier::SolCtxModifier, registers::SolStandardRegister, SolAffecteeFilter, SolDomain, SolLocationKind,
        SolRawModifier,
    },
    SolView,
};

use super::{reg_cmod, unreg_cmod};

impl SolStandardRegister {
    pub(in crate::sol::svc::svce_calc) fn reg_fw_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<SolCtxModifier>,
        sol_view: &SolView,
        fw_effect: &SolFwEffect,
        raw_modifier: SolRawModifier,
    ) {
        ctx_modifiers.clear();
        let valid = match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Everything => {
                    let projectee_item_ids = self.affectee_buffable.get(&fw_effect.fit_id);
                    ctx_modifiers.reserve_exact(projectee_item_ids.len());
                    for projectee_item_id in projectee_item_ids {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, *projectee_item_id);
                        ctx_modifiers.push(ctx_modifier);
                        self.cmods_direct.add_entry(*projectee_item_id, ctx_modifier);
                    }
                    true
                }
                SolDomain::Ship => {
                    let fit = sol_view.fits.get_fit(&fw_effect.fit_id).unwrap();
                    if matches!(fit.kind, SolShipKind::Ship) {
                        if let Some(ship_id) = fit.ship {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            ctx_modifiers.push(ctx_modifier);
                            self.cmods_direct.add_entry(ship_id, ctx_modifier);
                        }
                    }
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Everything => {
                    let fit = sol_view.fits.get_fit(&fw_effect.fit_id).unwrap();
                    if let Ok(loc) = fit.kind.try_into() {
                        if let Some(ship_id) = fit.ship {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            ctx_modifiers.push(ctx_modifier);
                            self.cmods_loc.add_entry((fw_effect.fit_id, loc), ctx_modifier);
                        }
                    }
                    true
                }
                SolDomain::Ship => {
                    let fit = sol_view.fits.get_fit(&fw_effect.fit_id).unwrap();
                    if let Some(ship_id) = fit.ship {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            ctx_modifiers.push(ctx_modifier);
                            self.cmods_loc
                                .add_entry((fw_effect.fit_id, SolLocationKind::Ship), ctx_modifier);
                        }
                    }
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Everything => {
                    let fit = sol_view.fits.get_fit(&fw_effect.fit_id).unwrap();
                    if let Ok(loc) = fit.kind.try_into() {
                        if let Some(ship_id) = fit.ship {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            ctx_modifiers.push(ctx_modifier);
                            self.cmods_loc_grp
                                .add_entry((fw_effect.fit_id, loc, grp_id), ctx_modifier);
                        }
                    }
                    true
                }
                SolDomain::Ship => {
                    let fit = sol_view.fits.get_fit(&fw_effect.fit_id).unwrap();
                    if let Some(ship_id) = fit.ship {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            ctx_modifiers.push(ctx_modifier);
                            self.cmods_loc_grp
                                .add_entry((fw_effect.fit_id, SolLocationKind::Ship, grp_id), ctx_modifier);
                        }
                    }
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Everything => {
                    let fit = sol_view.fits.get_fit(&fw_effect.fit_id).unwrap();
                    if let Ok(loc) = fit.kind.try_into() {
                        if let Some(ship_id) = fit.ship {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            ctx_modifiers.push(ctx_modifier);
                            self.cmods_loc_srq
                                .add_entry((fw_effect.fit_id, loc, srq_id), ctx_modifier);
                        }
                    }
                    true
                }
                SolDomain::Ship => {
                    let fit = sol_view.fits.get_fit(&fw_effect.fit_id).unwrap();
                    if let Some(ship_id) = fit.ship {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            ctx_modifiers.push(ctx_modifier);
                            self.cmods_loc_srq
                                .add_entry((fw_effect.fit_id, SolLocationKind::Ship, srq_id), ctx_modifier);
                        }
                    }
                    true
                }
                _ => false,
            },
            _ => false,
        };
        if valid {
            self.rmods_nonproj
                .add_entry((raw_modifier.affector_item_id, raw_modifier.effect_id), raw_modifier);
            self.rmods_fw_buff.add_entry(fw_effect.fit_id, raw_modifier);
        }
        for ctx_modifier in ctx_modifiers.iter() {
            reg_cmod(&mut self.cmods_by_attr_spec, *ctx_modifier);
        }
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_fw_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<SolCtxModifier>,
        sol_view: &SolView,
        fw_effect: &SolFwEffect,
        raw_modifier: SolRawModifier,
    ) {
        ctx_modifiers.clear();
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Everything => {
                    let projectee_item_ids = self.affectee_buffable.get(&fw_effect.fit_id);
                    ctx_modifiers.reserve_exact(projectee_item_ids.len());
                    for projectee_item_id in projectee_item_ids {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, *projectee_item_id);
                        ctx_modifiers.push(ctx_modifier);
                        self.cmods_direct.remove_entry(projectee_item_id, &ctx_modifier);
                    }
                }
                SolDomain::Ship => {
                    let fit = sol_view.fits.get_fit(&fw_effect.fit_id).unwrap();
                    if matches!(fit.kind, SolShipKind::Ship) {
                        if let Some(ship_id) = fit.ship {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            ctx_modifiers.push(ctx_modifier);
                            self.cmods_direct.remove_entry(&ship_id, &ctx_modifier);
                        }
                    }
                }
                _ => (),
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Everything => {
                    let fit = sol_view.fits.get_fit(&fw_effect.fit_id).unwrap();
                    if let Ok(loc) = fit.kind.try_into() {
                        if let Some(ship_id) = fit.ship {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            ctx_modifiers.push(ctx_modifier);
                            self.cmods_loc.remove_entry(&(fw_effect.fit_id, loc), &ctx_modifier);
                        }
                    }
                }
                SolDomain::Ship => {
                    let fit = sol_view.fits.get_fit(&fw_effect.fit_id).unwrap();
                    if let Some(ship_id) = fit.ship {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            ctx_modifiers.push(ctx_modifier);
                            self.cmods_loc
                                .remove_entry(&(fw_effect.fit_id, SolLocationKind::Ship), &ctx_modifier);
                        }
                    }
                }
                _ => (),
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Everything => {
                    let fit = sol_view.fits.get_fit(&fw_effect.fit_id).unwrap();
                    if let Ok(loc) = fit.kind.try_into() {
                        if let Some(ship_id) = fit.ship {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            ctx_modifiers.push(ctx_modifier);
                            self.cmods_loc_grp
                                .remove_entry(&(fw_effect.fit_id, loc, grp_id), &ctx_modifier);
                        }
                    }
                }
                SolDomain::Ship => {
                    let fit = sol_view.fits.get_fit(&fw_effect.fit_id).unwrap();
                    if let Some(ship_id) = fit.ship {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            ctx_modifiers.push(ctx_modifier);
                            self.cmods_loc_grp
                                .remove_entry(&(fw_effect.fit_id, SolLocationKind::Ship, grp_id), &ctx_modifier);
                        }
                    }
                }
                _ => (),
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Everything => {
                    let fit = sol_view.fits.get_fit(&fw_effect.fit_id).unwrap();
                    if let Ok(loc) = fit.kind.try_into() {
                        if let Some(ship_id) = fit.ship {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            ctx_modifiers.push(ctx_modifier);
                            self.cmods_loc_srq
                                .remove_entry(&(fw_effect.fit_id, loc, srq_id), &ctx_modifier);
                        }
                    }
                }
                SolDomain::Ship => {
                    let fit = sol_view.fits.get_fit(&fw_effect.fit_id).unwrap();
                    if let Some(ship_id) = fit.ship {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            ctx_modifiers.push(ctx_modifier);
                            self.cmods_loc_srq
                                .remove_entry(&(fw_effect.fit_id, SolLocationKind::Ship, srq_id), &ctx_modifier);
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
        self.rmods_fw_buff.remove_entry(&fw_effect.fit_id, &raw_modifier);
        for ctx_modifier in ctx_modifiers.iter() {
            unreg_cmod(&mut self.cmods_by_attr_spec, ctx_modifier);
        }
    }
    // Is supposed to be called only for buffable items
    pub(in crate::sol::svc::svce_calc::registers::standard) fn reg_buffable_for_fw(&mut self, item: &SolItem) {
        let fit_id = match item.get_fit_id() {
            Some(fit_id) => fit_id,
            None => return,
        };
        for raw_modifier in self.rmods_fw_buff.get(&fit_id) {
            match raw_modifier.affectee_filter {
                SolAffecteeFilter::Direct(dom) => match dom {
                    SolDomain::Everything => {
                        let item_id = item.get_id();
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, item_id);
                        self.cmods_direct.add_entry(item_id, ctx_modifier);
                    }
                    SolDomain::Ship => {
                        if let SolItem::Ship(ship) = item {
                            if matches!(ship.kind, SolShipKind::Ship) {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.id);
                                self.cmods_direct.add_entry(ship.id, ctx_modifier);
                            }
                        }
                    }
                    _ => (),
                },
                SolAffecteeFilter::Loc(dom) => match dom {
                    SolDomain::Everything => {
                        if let SolItem::Ship(ship) = item {
                            if let Ok(loc) = ship.kind.try_into() {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.id);
                                self.cmods_loc.add_entry((ship.fit_id, loc), ctx_modifier);
                            }
                        }
                    }
                    SolDomain::Ship => {
                        if let SolItem::Ship(ship) = item {
                            if matches!(ship.kind, SolShipKind::Ship) {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.id);
                                self.cmods_loc
                                    .add_entry((ship.fit_id, SolLocationKind::Ship), ctx_modifier);
                            }
                        }
                    }
                    _ => (),
                },
                SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                    SolDomain::Everything => {
                        if let SolItem::Ship(ship) = item {
                            if let Ok(loc) = ship.kind.try_into() {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.id);
                                self.cmods_loc_grp.add_entry((ship.fit_id, loc, grp_id), ctx_modifier);
                            }
                        }
                    }
                    SolDomain::Ship => {
                        if let SolItem::Ship(ship) = item {
                            if matches!(ship.kind, SolShipKind::Ship) {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.id);
                                self.cmods_loc_grp
                                    .add_entry((ship.fit_id, SolLocationKind::Ship, grp_id), ctx_modifier);
                            }
                        }
                    }
                    _ => (),
                },
                SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                    SolDomain::Everything => {
                        if let SolItem::Ship(ship) = item {
                            if let Ok(loc) = ship.kind.try_into() {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.id);
                                self.cmods_loc_srq.add_entry((ship.fit_id, loc, srq_id), ctx_modifier);
                            }
                        }
                    }
                    SolDomain::Ship => {
                        if let SolItem::Ship(ship) = item {
                            if matches!(ship.kind, SolShipKind::Ship) {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.id);
                                self.cmods_loc_srq
                                    .add_entry((ship.fit_id, SolLocationKind::Ship, srq_id), ctx_modifier);
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            };
        }
    }
    // Is supposed to be called only for buffable items
    pub(in crate::sol::svc::svce_calc::registers::standard) fn unreg_buffable_for_fw(&mut self, item: &SolItem) {
        let fit_id = match item.get_fit_id() {
            Some(fit_id) => fit_id,
            None => return,
        };
        for raw_modifier in self.rmods_fw_buff.get(&fit_id) {
            match raw_modifier.affectee_filter {
                SolAffecteeFilter::Direct(dom) => match dom {
                    SolDomain::Everything => {
                        let item_id = item.get_id();
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, item_id);
                        self.cmods_direct.remove_entry(&item_id, &ctx_modifier);
                    }
                    SolDomain::Ship => {
                        if let SolItem::Ship(ship) = item {
                            if matches!(ship.kind, SolShipKind::Ship) {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.id);
                                self.cmods_direct.remove_entry(&ship.id, &ctx_modifier);
                            }
                        }
                    }
                    _ => (),
                },
                SolAffecteeFilter::Loc(dom) => match dom {
                    SolDomain::Everything => {
                        if let SolItem::Ship(ship) = item {
                            if let Ok(loc) = ship.kind.try_into() {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.id);
                                self.cmods_loc.remove_entry(&(ship.fit_id, loc), &ctx_modifier);
                            }
                        }
                    }
                    SolDomain::Ship => {
                        if let SolItem::Ship(ship) = item {
                            if matches!(ship.kind, SolShipKind::Ship) {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.id);
                                self.cmods_loc
                                    .remove_entry(&(ship.fit_id, SolLocationKind::Ship), &ctx_modifier);
                            }
                        }
                    }
                    _ => (),
                },
                SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                    SolDomain::Everything => {
                        if let SolItem::Ship(ship) = item {
                            if let Ok(loc) = ship.kind.try_into() {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.id);
                                self.cmods_loc_grp
                                    .remove_entry(&(ship.fit_id, loc, grp_id), &ctx_modifier);
                            }
                        }
                    }
                    SolDomain::Ship => {
                        if let SolItem::Ship(ship) = item {
                            if matches!(ship.kind, SolShipKind::Ship) {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.id);
                                self.cmods_loc_grp
                                    .remove_entry(&(ship.fit_id, SolLocationKind::Ship, grp_id), &ctx_modifier);
                            }
                        }
                    }
                    _ => (),
                },
                SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                    SolDomain::Everything => {
                        if let SolItem::Ship(ship) = item {
                            if let Ok(loc) = ship.kind.try_into() {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.id);
                                self.cmods_loc_srq
                                    .remove_entry(&(ship.fit_id, loc, srq_id), &ctx_modifier);
                            }
                        }
                    }
                    SolDomain::Ship => {
                        if let SolItem::Ship(ship) = item {
                            if matches!(ship.kind, SolShipKind::Ship) {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.id);
                                self.cmods_loc_srq
                                    .remove_entry(&(ship.fit_id, SolLocationKind::Ship, srq_id), &ctx_modifier);
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            };
        }
    }
}
