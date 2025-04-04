use crate::sol::{
    svc::calc::{
        AffecteeFilter, Location, LocationKind, RawModifier, modifier::CtxModifier, registers::StandardRegister,
    },
    uad::{
        Uad,
        item::{FwEffect, Item, ShipKind},
    },
};

use super::{add_ctx_modifier, remove_ctx_modifier};

impl StandardRegister {
    pub(in crate::sol::svc::calc) fn reg_fw_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<CtxModifier>,
        uad: &Uad,
        fw_effect: &FwEffect,
        raw_modifier: RawModifier,
    ) -> bool {
        ctx_modifiers.clear();
        let valid = match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => {
                    let projectee_item_ids = self.affectee_buffable.get(&fw_effect.get_fit_id());
                    ctx_modifiers.reserve(projectee_item_ids.len());
                    for projectee_item_id in projectee_item_ids {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, *projectee_item_id);
                        add_ctx_modifier(
                            &mut self.cmods_direct,
                            *projectee_item_id,
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier)
                    }
                    true
                }
                Location::Ship => {
                    let fit = uad.fits.get_fit(&fw_effect.get_fit_id()).unwrap();
                    if matches!(fit.kind, ShipKind::Ship) {
                        if let Some(ship_id) = fit.ship {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            add_ctx_modifier(
                                &mut self.cmods_root,
                                (fit.id, LocationKind::Ship),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            ctx_modifiers.push(ctx_modifier);
                        }
                    }
                    true
                }
                _ => false,
            },
            AffecteeFilter::Loc(Location::Everything | Location::Ship) => {
                let fit = uad.fits.get_fit(&fw_effect.get_fit_id()).unwrap();
                if let Some(ship_id) = fit.ship {
                    if matches!(fit.kind, ShipKind::Ship) {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_id);
                        add_ctx_modifier(
                            &mut self.cmods_loc,
                            (fw_effect.get_fit_id(), LocationKind::Ship),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
                true
            }
            AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => {
                let fit = uad.fits.get_fit(&fw_effect.get_fit_id()).unwrap();
                if let Some(ship_id) = fit.ship {
                    if matches!(fit.kind, ShipKind::Ship) {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_id);
                        add_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            (fw_effect.get_fit_id(), LocationKind::Ship, a_item_grp_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
                true
            }
            AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => {
                let fit = uad.fits.get_fit(&fw_effect.get_fit_id()).unwrap();
                if let Some(ship_id) = fit.ship {
                    if matches!(fit.kind, ShipKind::Ship) {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_id);
                        add_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            (fw_effect.get_fit_id(), LocationKind::Ship, srq_a_item_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
                true
            }
            _ => false,
        };
        if valid {
            self.rmods_nonproj
                .add_entry((raw_modifier.affector_item_id, raw_modifier.a_effect_id), raw_modifier);
            self.rmods_fw_buff.add_entry(fw_effect.get_fit_id(), raw_modifier);
        }
        valid
    }
    pub(in crate::sol::svc::calc) fn unreg_fw_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<CtxModifier>,
        uad: &Uad,
        fw_effect: &FwEffect,
        raw_modifier: RawModifier,
    ) {
        ctx_modifiers.clear();
        match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => {
                    let projectee_item_ids = self.affectee_buffable.get(&fw_effect.get_fit_id());
                    ctx_modifiers.reserve(projectee_item_ids.len());
                    for projectee_item_id in projectee_item_ids {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, *projectee_item_id);
                        remove_ctx_modifier(
                            &mut self.cmods_direct,
                            projectee_item_id,
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
                Location::Ship => {
                    let fit = uad.fits.get_fit(&fw_effect.get_fit_id()).unwrap();
                    if matches!(fit.kind, ShipKind::Ship) {
                        if let Some(ship_id) = fit.ship {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            remove_ctx_modifier(
                                &mut self.cmods_root,
                                &(fit.id, LocationKind::Ship),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            ctx_modifiers.push(ctx_modifier);
                        }
                    }
                }
                _ => (),
            },
            AffecteeFilter::Loc(Location::Everything | Location::Ship) => {
                let fit = uad.fits.get_fit(&fw_effect.get_fit_id()).unwrap();
                if let Some(ship_id) = fit.ship {
                    if matches!(fit.kind, ShipKind::Ship) {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_id);
                        remove_ctx_modifier(
                            &mut self.cmods_loc,
                            &(fw_effect.get_fit_id(), LocationKind::Ship),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
            }
            AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => {
                let fit = uad.fits.get_fit(&fw_effect.get_fit_id()).unwrap();
                if let Some(ship_id) = fit.ship {
                    if matches!(fit.kind, ShipKind::Ship) {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_id);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            &(fw_effect.get_fit_id(), LocationKind::Ship, a_item_grp_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
            }
            AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => {
                let fit = uad.fits.get_fit(&fw_effect.get_fit_id()).unwrap();
                if let Some(ship_id) = fit.ship {
                    if matches!(fit.kind, ShipKind::Ship) {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_id);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            &(fw_effect.get_fit_id(), LocationKind::Ship, srq_a_item_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
            }
            _ => (),
        }
        self.rmods_fw_buff.remove_entry(&fw_effect.get_fit_id(), &raw_modifier);
    }
    // Is supposed to be called only for buffable items
    pub(in crate::sol::svc::calc::registers::standard) fn reg_buffable_for_fw(&mut self, item: &Item) {
        let fit_id = match item.get_fit_id() {
            Some(fit_id) => fit_id,
            None => return,
        };
        for raw_modifier in self.rmods_fw_buff.get(&fit_id) {
            match raw_modifier.affectee_filter {
                AffecteeFilter::Direct(loc) => match loc {
                    Location::Everything => {
                        let item_id = item.get_item_id();
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_id);
                        add_ctx_modifier(
                            &mut self.cmods_direct,
                            item_id,
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                    Location::Ship => {
                        if let Item::Ship(ship) = item {
                            if matches!(ship.get_kind(), ShipKind::Ship) {
                                let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, ship.get_item_id());
                                add_ctx_modifier(
                                    &mut self.cmods_root,
                                    (ship.get_fit_id(), LocationKind::Ship),
                                    ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                            }
                        }
                    }
                    _ => (),
                },
                AffecteeFilter::Loc(Location::Everything | Location::Ship) => {
                    if let Item::Ship(ship) = item {
                        if matches!(ship.get_kind(), ShipKind::Ship) {
                            let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, ship.get_item_id());
                            add_ctx_modifier(
                                &mut self.cmods_loc,
                                (ship.get_fit_id(), LocationKind::Ship),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                        }
                    }
                }
                AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => {
                    if let Item::Ship(ship) = item {
                        if matches!(ship.get_kind(), ShipKind::Ship) {
                            let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, ship.get_item_id());
                            add_ctx_modifier(
                                &mut self.cmods_loc_grp,
                                (ship.get_fit_id(), LocationKind::Ship, a_item_grp_id),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                        }
                    }
                }
                AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => {
                    if let Item::Ship(ship) = item {
                        if matches!(ship.get_kind(), ShipKind::Ship) {
                            let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, ship.get_item_id());
                            add_ctx_modifier(
                                &mut self.cmods_loc_srq,
                                (ship.get_fit_id(), LocationKind::Ship, srq_a_item_id),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                        }
                    }
                }
                _ => (),
            };
        }
    }
    // Is supposed to be called only for buffable items
    pub(in crate::sol::svc::calc::registers::standard) fn unreg_buffable_for_fw(&mut self, item: &Item) {
        let fit_id = match item.get_fit_id() {
            Some(fit_id) => fit_id,
            None => return,
        };
        for raw_modifier in self.rmods_fw_buff.get(&fit_id) {
            match raw_modifier.affectee_filter {
                AffecteeFilter::Direct(loc) => match loc {
                    Location::Everything => {
                        let item_id = item.get_item_id();
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_id);
                        remove_ctx_modifier(
                            &mut self.cmods_direct,
                            &item_id,
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                    Location::Ship => {
                        if let Item::Ship(ship) = item {
                            if matches!(ship.get_kind(), ShipKind::Ship) {
                                let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, ship.get_item_id());
                                remove_ctx_modifier(
                                    &mut self.cmods_root,
                                    &(ship.get_fit_id(), LocationKind::Ship),
                                    &ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                            }
                        }
                    }
                    _ => (),
                },
                AffecteeFilter::Loc(Location::Everything | Location::Ship) => {
                    if let Item::Ship(ship) = item {
                        if matches!(ship.get_kind(), ShipKind::Ship) {
                            let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, ship.get_item_id());
                            remove_ctx_modifier(
                                &mut self.cmods_loc,
                                &(ship.get_fit_id(), LocationKind::Ship),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                        }
                    }
                }
                AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => {
                    if let Item::Ship(ship) = item {
                        if matches!(ship.get_kind(), ShipKind::Ship) {
                            let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, ship.get_item_id());
                            remove_ctx_modifier(
                                &mut self.cmods_loc_grp,
                                &(ship.get_fit_id(), LocationKind::Ship, a_item_grp_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                        }
                    }
                }
                AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => {
                    if let Item::Ship(ship) = item {
                        if matches!(ship.get_kind(), ShipKind::Ship) {
                            let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, ship.get_item_id());
                            remove_ctx_modifier(
                                &mut self.cmods_loc_srq,
                                &(ship.get_fit_id(), LocationKind::Ship, srq_a_item_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                        }
                    }
                }
                _ => (),
            };
        }
    }
}
