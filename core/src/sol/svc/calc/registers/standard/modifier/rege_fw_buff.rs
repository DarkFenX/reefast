use super::{add_ctx_modifier, remove_ctx_modifier};
use crate::sol::{
    FitKey, ItemKey,
    svc::calc::{
        AffecteeFilter, Location, LocationKind, RawModifier, modifier::CtxModifier, registers::StandardRegister,
    },
    uad::{
        Uad,
        item::{ShipKind, UadFwEffect, UadItem},
    },
};

impl StandardRegister {
    pub(in crate::sol::svc::calc) fn reg_fw_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<CtxModifier>,
        uad: &Uad,
        fw_effect: &UadFwEffect,
        raw_modifier: RawModifier,
    ) -> bool {
        ctx_modifiers.clear();
        let valid = match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => {
                    let affectee_item_keys = self.affectee_buffable.get(&fw_effect.get_fit_key());
                    ctx_modifiers.reserve(affectee_item_keys.len());
                    for &affectee_item_key in affectee_item_keys {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, affectee_item_key);
                        add_ctx_modifier(
                            &mut self.cmods_direct,
                            affectee_item_key,
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier)
                    }
                    self.rmods_fw_buff_direct
                        .add_entry(fw_effect.get_fit_key(), raw_modifier);
                    true
                }
                Location::Ship => {
                    let fit_key = fw_effect.get_fit_key();
                    let fit = uad.fits.get(fit_key);
                    if matches!(fit.kind, ShipKind::Ship)
                        && let Some(ship_key) = fit.ship
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_key);
                        add_ctx_modifier(
                            &mut self.cmods_direct,
                            ship_key,
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                    self.rmods_fw_buff_direct
                        .add_entry(fw_effect.get_fit_key(), raw_modifier);
                    true
                }
                _ => false,
            },
            AffecteeFilter::Loc(Location::Everything | Location::Ship) => {
                let fit_key = fw_effect.get_fit_key();
                let fit = uad.fits.get(fit_key);
                if let Some(ship_key) = fit.ship
                    && matches!(fit.kind, ShipKind::Ship)
                {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_key);
                    add_ctx_modifier(
                        &mut self.cmods_loc,
                        (fit_key, LocationKind::Ship),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    ctx_modifiers.push(ctx_modifier);
                }
                self.rmods_fw_buff_indirect
                    .add_entry(fw_effect.get_fit_key(), raw_modifier);
                true
            }
            AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => {
                let fit_key = fw_effect.get_fit_key();
                let fit = uad.fits.get(fit_key);
                if let Some(ship_key) = fit.ship
                    && matches!(fit.kind, ShipKind::Ship)
                {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_key);
                    add_ctx_modifier(
                        &mut self.cmods_loc_grp,
                        (fit_key, LocationKind::Ship, a_item_grp_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    ctx_modifiers.push(ctx_modifier);
                }
                self.rmods_fw_buff_indirect
                    .add_entry(fw_effect.get_fit_key(), raw_modifier);
                true
            }
            AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => {
                let fit_key = fw_effect.get_fit_key();
                let fit = uad.fits.get(fit_key);
                if let Some(ship_key) = fit.ship
                    && matches!(fit.kind, ShipKind::Ship)
                {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_key);
                    add_ctx_modifier(
                        &mut self.cmods_loc_srq,
                        (fit_key, LocationKind::Ship, srq_a_item_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    ctx_modifiers.push(ctx_modifier);
                }
                self.rmods_fw_buff_indirect
                    .add_entry(fw_effect.get_fit_key(), raw_modifier);
                true
            }
            _ => false,
        };
        if valid {
            self.rmods_all.add_entry(raw_modifier.affector_espec, raw_modifier);
        }
        valid
    }
    pub(in crate::sol::svc::calc) fn unreg_fw_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<CtxModifier>,
        uad: &Uad,
        fw_effect: &UadFwEffect,
        raw_modifier: RawModifier,
    ) {
        ctx_modifiers.clear();
        match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => {
                    let affectee_item_keys = self.affectee_buffable.get(&fw_effect.get_fit_key());
                    ctx_modifiers.reserve(affectee_item_keys.len());
                    for affectee_item_key in affectee_item_keys {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, *affectee_item_key);
                        remove_ctx_modifier(
                            &mut self.cmods_direct,
                            affectee_item_key,
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                    self.rmods_fw_buff_direct
                        .remove_entry(&fw_effect.get_fit_key(), &raw_modifier);
                }
                Location::Ship => {
                    let fit_key = fw_effect.get_fit_key();
                    let fit = uad.fits.get(fit_key);
                    if matches!(fit.kind, ShipKind::Ship)
                        && let Some(ship_key) = fit.ship
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_key);
                        remove_ctx_modifier(
                            &mut self.cmods_direct,
                            &ship_key,
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                    self.rmods_fw_buff_direct
                        .remove_entry(&fw_effect.get_fit_key(), &raw_modifier);
                }
                _ => (),
            },
            AffecteeFilter::Loc(Location::Everything | Location::Ship) => {
                let fit_key = fw_effect.get_fit_key();
                let fit = uad.fits.get(fit_key);
                if let Some(ship_key) = fit.ship
                    && matches!(fit.kind, ShipKind::Ship)
                {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_key);
                    remove_ctx_modifier(
                        &mut self.cmods_loc,
                        &(fit_key, LocationKind::Ship),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    ctx_modifiers.push(ctx_modifier);
                }
                self.rmods_fw_buff_indirect
                    .remove_entry(&fw_effect.get_fit_key(), &raw_modifier);
            }
            AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => {
                let fit_key = fw_effect.get_fit_key();
                let fit = uad.fits.get(fit_key);
                if let Some(ship_key) = fit.ship
                    && matches!(fit.kind, ShipKind::Ship)
                {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_key);
                    remove_ctx_modifier(
                        &mut self.cmods_loc_grp,
                        &(fit_key, LocationKind::Ship, a_item_grp_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    ctx_modifiers.push(ctx_modifier);
                }
                self.rmods_fw_buff_indirect
                    .remove_entry(&fw_effect.get_fit_key(), &raw_modifier);
            }
            AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => {
                let fit_key = fw_effect.get_fit_key();
                let fit = uad.fits.get(fit_key);
                if let Some(ship_key) = fit.ship
                    && matches!(fit.kind, ShipKind::Ship)
                {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_key);
                    remove_ctx_modifier(
                        &mut self.cmods_loc_srq,
                        &(fit_key, LocationKind::Ship, srq_a_item_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    ctx_modifiers.push(ctx_modifier);
                }
                self.rmods_fw_buff_indirect
                    .remove_entry(&fw_effect.get_fit_key(), &raw_modifier);
            }
            _ => (),
        }
    }
    // Is supposed to be called only for buffable items
    pub(in crate::sol::svc::calc::registers::standard) fn reg_buffable_for_fw(
        &mut self,
        item_key: ItemKey,
        item: &UadItem,
        fit_key: FitKey,
    ) {
        for raw_modifier in self.rmods_fw_buff_direct.get(&fit_key) {
            if let AffecteeFilter::Direct(loc) = raw_modifier.affectee_filter {
                match loc {
                    Location::Everything => {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                        add_ctx_modifier(
                            &mut self.cmods_direct,
                            item_key,
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                    Location::Ship => {
                        if let UadItem::Ship(ship) = item
                            && matches!(ship.get_kind(), ShipKind::Ship)
                        {
                            let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                            add_ctx_modifier(
                                &mut self.cmods_direct,
                                item_key,
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    // Is supposed to be called only for buffable items
    pub(in crate::sol::svc::calc::registers::standard) fn unreg_buffable_for_fw(
        &mut self,
        item_key: ItemKey,
        item: &UadItem,
        fit_key: FitKey,
    ) {
        for raw_modifier in self.rmods_fw_buff_direct.get(&fit_key) {
            if let AffecteeFilter::Direct(loc) = raw_modifier.affectee_filter {
                match loc {
                    Location::Everything => {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                        remove_ctx_modifier(
                            &mut self.cmods_direct,
                            &item_key,
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                    Location::Ship => {
                        if let UadItem::Ship(ship) = item
                            && matches!(ship.get_kind(), ShipKind::Ship)
                        {
                            let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_direct,
                                &item_key,
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    // Is supposed to be called only for buffable location roots (ships)
    pub(super) fn reg_loc_root_for_fw_buff(&mut self, item_key: ItemKey, item: &UadItem, fit_key: FitKey) {
        for raw_modifier in self.rmods_fw_buff_indirect.get(&fit_key) {
            match raw_modifier.affectee_filter {
                AffecteeFilter::Loc(Location::Everything | Location::Ship) => {
                    if let UadItem::Ship(ship) = item
                        && matches!(ship.get_kind(), ShipKind::Ship)
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                        add_ctx_modifier(
                            &mut self.cmods_loc,
                            (ship.get_fit_key(), LocationKind::Ship),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => {
                    if let UadItem::Ship(ship) = item
                        && matches!(ship.get_kind(), ShipKind::Ship)
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                        add_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            (ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => {
                    if let UadItem::Ship(ship) = item
                        && matches!(ship.get_kind(), ShipKind::Ship)
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                        add_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            (ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                _ => (),
            };
        }
    }
    // Is supposed to be called only for buffable location roots (ships)
    pub(super) fn unreg_loc_root_for_fw_buff(&mut self, item_key: ItemKey, item: &UadItem, fit_key: FitKey) {
        for raw_modifier in self.rmods_fw_buff_indirect.get(&fit_key) {
            match raw_modifier.affectee_filter {
                AffecteeFilter::Loc(Location::Everything | Location::Ship) => {
                    if let UadItem::Ship(ship) = item
                        && matches!(ship.get_kind(), ShipKind::Ship)
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                        remove_ctx_modifier(
                            &mut self.cmods_loc,
                            &(ship.get_fit_key(), LocationKind::Ship),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => {
                    if let UadItem::Ship(ship) = item
                        && matches!(ship.get_kind(), ShipKind::Ship)
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            &(ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => {
                    if let UadItem::Ship(ship) = item
                        && matches!(ship.get_kind(), ShipKind::Ship)
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            &(ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                _ => (),
            };
        }
    }
}
