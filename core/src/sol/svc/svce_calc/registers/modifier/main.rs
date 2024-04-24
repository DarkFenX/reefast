use std::{convert::TryInto, hash::Hash};

use itertools::Itertools;

use crate::{
    defs::{EAttrId, EItemGrpId, EItemId, SolFitId, SolItemId},
    sol::{
        fit::SolFits,
        fleet::SolFleet,
        item::SolItem,
        svc::svce_calc::{SolAffecteeFilter, SolAttrMod, SolFleetUpdates, SolLocType, SolModDomain, SolModType},
        SolView,
    },
    util::{StMapSetL1, StSet},
};

use super::ActiveLocations;

pub(in crate::sol::svc::svce_calc) struct SolModifierRegister {
    // Modifiers registered for an item
    // Map<affector item ID, modifiers>
    pub(super) by_affector: StMapSetL1<SolItemId, SolAttrMod>,
    // Modifiers which modify item directly
    // Map<affectee item ID, modifiers>
    pub(super) direct: StMapSetL1<SolItemId, SolAttrMod>,
    // Modifiers which modify 'other' domain are always stored here, regardless if they actually
    // modify something or not
    // Map<affector item ID, modifiers>
    pub(super) other: StMapSetL1<SolItemId, SolAttrMod>,
    // All modifiers which modify root entities (via ship or character reference) are kept here
    // Map<(affectee fit ID, affectee location type), modifiers>
    pub(super) root: StMapSetL1<(SolFitId, SolLocType), SolAttrMod>,
    // Modifiers influencing all items belonging to certain fit and location type
    // Map<(affectee fit ID, affectee location type), modifiers>
    pub(super) loc: StMapSetL1<(SolFitId, SolLocType), SolAttrMod>,
    // Modifiers influencing items belonging to certain fit, location and group
    // Map<(affectee fit ID, affectee location, affectee group ID), modifiers>
    pub(super) loc_grp: StMapSetL1<(SolFitId, SolLocType, EItemGrpId), SolAttrMod>,
    // Modifiers influencing items belonging to certain fit and location, and having certain skill
    // requirement
    // Map<(affectee fit ID, affectee location, affectee skillreq type ID), modifiers>
    pub(super) loc_srq: StMapSetL1<(SolFitId, SolLocType, EItemId), SolAttrMod>,
    // Modifiers influencing owner-modifiable items belonging to certain fit and having certain
    // skill requirement
    // Map<(affectee fit ID, affectee skillreq type ID), modifiers>
    pub(super) own_srq: StMapSetL1<(SolFitId, EItemId), SolAttrMod>,
    // Modifiers influencing all buff-modifiable items
    // Map<affectee fit ID, modifiers>
    pub(super) buff_all: StMapSetL1<SolFitId, SolAttrMod>,
    // Fleet modifiers on a per-fit basis
    // Map<source fit ID, modifiers>
    pub(super) fleet_fit: StMapSetL1<SolFitId, SolAttrMod>,
    // System-wide modifiers
    pub(super) sw: StSet<SolAttrMod>,
}
impl SolModifierRegister {
    pub(in crate::sol::svc::svce_calc) fn new() -> Self {
        Self {
            by_affector: StMapSetL1::new(),
            direct: StMapSetL1::new(),
            other: StMapSetL1::new(),
            root: StMapSetL1::new(),
            loc: StMapSetL1::new(),
            loc_grp: StMapSetL1::new(),
            loc_srq: StMapSetL1::new(),
            own_srq: StMapSetL1::new(),
            buff_all: StMapSetL1::new(),
            fleet_fit: StMapSetL1::new(),
            sw: StSet::new(),
        }
    }
    // Query methods
    pub(in crate::sol::svc::svce_calc) fn get_mods_for_affectee(
        &self,
        item: &SolItem,
        attr_id: &EAttrId,
        fits: &SolFits,
    ) -> Vec<SolAttrMod> {
        let item_id = item.get_id();
        let fit_opt = item.get_fit_id().map(|v| fits.get_fit(&v).ok()).flatten();
        let root_loc_opt = item.get_root_loc_type();
        let grp_id_opt = item.get_group_id().ok();
        let srqs_opt = item.get_skill_reqs().ok();
        let mut mods = Vec::new();
        filter_and_extend(&mut mods, &self.direct, &item_id, attr_id);
        if let Some(other_item_id) = item.get_other() {
            filter_and_extend(&mut mods, &self.other, &other_item_id, attr_id);
        }
        if let (Some(fit), Some(root_loc)) = (fit_opt, root_loc_opt) {
            filter_and_extend(&mut mods, &self.root, &(fit.id, root_loc), attr_id);
        }
        if let Some(fit) = fit_opt {
            for loc_type in ActiveLocations::new(item, fit) {
                filter_and_extend(&mut mods, &self.loc, &(fit.id, loc_type), attr_id);
            }
        }
        if let (Some(fit), Some(grp_id)) = (fit_opt, grp_id_opt) {
            for loc_type in ActiveLocations::new(item, fit) {
                filter_and_extend(&mut mods, &self.loc_grp, &(fit.id, loc_type, grp_id), attr_id);
            }
        }
        if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
            for loc_type in ActiveLocations::new(item, fit) {
                for srq_id in srqs.keys() {
                    filter_and_extend(&mut mods, &self.loc_srq, &(fit.id, loc_type, *srq_id), attr_id);
                }
            }
        }
        if item.is_owner_modifiable() {
            if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
                for srq_id in srqs.keys() {
                    filter_and_extend(&mut mods, &self.own_srq, &(fit.id, *srq_id), attr_id);
                }
            }
        }
        if item.is_buff_modifiable() {
            if let Some(fit) = fit_opt {
                filter_and_extend(&mut mods, &self.buff_all, &fit.id, attr_id);
            }
        }
        mods
    }
    pub(in crate::sol::svc::svce_calc) fn get_mods_for_changed_root(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
    ) -> Vec<SolAttrMod> {
        let mut mods = Vec::new();
        if let (Some(_), Some(loc_type)) = (item.get_fit_id(), item.get_root_loc_type()) {
            for (sub_item_id, sub_mods) in self.by_affector.iter() {
                if let Ok(_) = sol_view.items.get_item(sub_item_id) {
                    // TODO: This should be refined/optimized. It should pick only modifiers which
                    // TODO: target fit of item being changed.
                    for sub_mod in sub_mods {
                        if match sub_mod.affectee_filter {
                            SolAffecteeFilter::Loc(sub_dom) => compare_loc_dom(loc_type, sub_dom),
                            SolAffecteeFilter::LocGrp(sub_dom, _) => compare_loc_dom(loc_type, sub_dom),
                            SolAffecteeFilter::LocSrq(sub_dom, _) => compare_loc_dom(loc_type, sub_dom),
                            _ => false,
                        } {
                            mods.push(*sub_mod);
                        }
                    }
                }
            }
        }
        mods
    }
    pub(in crate::sol::svc::svce_calc) fn iter_affector_item_mods(
        &self,
        affector_item_id: &SolItemId,
    ) -> impl ExactSizeIterator<Item = &SolAttrMod> {
        self.by_affector.get(affector_item_id)
    }
    // Modification methods
    pub(in crate::sol::svc::svce_calc) fn reg_fit(&mut self, fit_id: &SolFitId) {
        let sw_modifiers = self.sw.iter().map(|v| *v).collect_vec();
        if !sw_modifiers.is_empty() {
            let fit_ids = vec![*fit_id];
            for modifier in sw_modifiers.iter() {
                self.apply_mod_to_fits(*modifier, &fit_ids);
            }
        }
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_fit(&mut self, fit_id: &SolFitId) {
        let sw_modifiers = self.sw.iter().map(|v| *v).collect_vec();
        if !sw_modifiers.is_empty() {
            let fit_ids = vec![*fit_id];
            for modifier in sw_modifiers.iter() {
                self.unapply_mod_from_fits(modifier, &fit_ids);
            }
        }
    }
    pub(in crate::sol::svc::svce_calc) fn reg_fleet_for_fit(
        &mut self,
        fleet: &SolFleet,
        fit_id: &SolFitId,
    ) -> SolFleetUpdates {
        let updates = self.get_fleet_updates(fleet, fit_id);
        if !updates.incoming.is_empty() {
            let fit_ids = vec![*fit_id];
            for modifier in updates.incoming.iter() {
                self.apply_mod_to_fits(*modifier, &fit_ids);
            }
        }
        if !updates.outgoing.is_empty() {
            let fit_ids = fleet.iter_fits().map(|v| *v).filter(|v| v != fit_id).collect();
            for modifier in updates.outgoing.iter() {
                self.apply_mod_to_fits(*modifier, &fit_ids);
            }
        }
        updates
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_fleet_for_fit(
        &mut self,
        fleet: &SolFleet,
        fit_id: &SolFitId,
    ) -> SolFleetUpdates {
        let updates = self.get_fleet_updates(fleet, fit_id);
        if !updates.incoming.is_empty() {
            let fit_ids = vec![*fit_id];
            for modifier in updates.incoming.iter() {
                self.unapply_mod_from_fits(modifier, &fit_ids);
            }
        }
        if !updates.outgoing.is_empty() {
            let fit_ids = fleet.iter_fits().map(|v| *v).filter(|v| v != fit_id).collect();
            for modifier in updates.outgoing.iter() {
                self.unapply_mod_from_fits(modifier, &fit_ids);
            }
        }
        updates
    }
    pub(in crate::sol::svc::svce_calc) fn reg_mod(
        &mut self,
        fit_ids: &mut Vec<SolFitId>,
        sol_view: &SolView,
        item: &SolItem,
        modifier: SolAttrMod,
    ) -> bool {
        // Maintain helper data containers
        self.by_affector.add_entry(modifier.affector_item_id, modifier);
        if matches!(item, SolItem::SwEffect(_)) {
            self.sw.insert(modifier);
        }
        if matches!(modifier.mod_type, SolModType::FleetBuff) {
            if let Some(fit_id) = item.get_fit_id() {
                self.fleet_fit.add_entry(fit_id, modifier);
            }
        }
        // Process simple modifiers which do not rely on any extra context
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Item => {
                    self.direct.add_entry(modifier.affector_item_id, modifier);
                    return true;
                }
                SolModDomain::Other => {
                    self.other.add_entry(modifier.affector_item_id, modifier);
                    return true;
                }
                _ => (),
            },
            _ => (),
        }
        match (modifier.mod_type, item) {
            // System-wide modifications affect all fits
            (SolModType::System | SolModType::Buff, SolItem::SwEffect(_)) => {
                fit_ids.clear();
                fit_ids.extend(sol_view.fits.iter_fit_ids());
                self.apply_mod_to_fits(modifier, fit_ids)
            }
            // Fit-wide modifications affect only source fit itself
            (SolModType::System | SolModType::Buff, SolItem::FwEffect(fw_effect)) => {
                fit_ids.clear();
                fit_ids.push(fw_effect.fit_id);
                self.apply_mod_to_fits(modifier, fit_ids)
            }
            // Local modifications are the same
            (SolModType::Local, _) => match item.get_fit_id() {
                Some(fit_id) => {
                    fit_ids.clear();
                    fit_ids.push(fit_id);
                    self.apply_mod_to_fits(modifier, fit_ids)
                }
                None => false,
            },
            // Fleet modifications affect whole fleet, or just source fit itself, if fleet isn't set
            (SolModType::FleetBuff, SolItem::Module(module)) => {
                fill_fleet_fits(fit_ids, sol_view, module.fit_id);
                self.apply_mod_to_fits(modifier, fit_ids)
            }
            // Various targetable effects affect only what they are target, depending on mod type
            (SolModType::System, SolItem::ProjEffect(proj_effect)) => {
                let mut changed = false;
                for tgt_item_id in proj_effect.tgts.iter_tgts() {
                    let tgt_item = sol_view.items.get_item(tgt_item_id).unwrap();
                    changed = changed | self.apply_system_mod_to_item(modifier, tgt_item);
                }
                changed
            }
            (SolModType::Targeted, _) => {
                let mut changed = false;
                if let Some(tgt_item_ids) = item.iter_targets() {
                    for tgt_item_id in tgt_item_ids {
                        let tgt_item = sol_view.items.get_item(tgt_item_id).unwrap();
                        changed = changed | self.apply_targeted_mod_to_item(modifier, tgt_item);
                    }
                }
                changed
            }
            (SolModType::Buff, _) => {
                let mut changed = false;
                if let Some(tgt_item_ids) = item.iter_targets() {
                    for tgt_item_id in tgt_item_ids {
                        let tgt_item = sol_view.items.get_item(tgt_item_id).unwrap();
                        changed = changed | self.apply_buff_mod_to_item(modifier, tgt_item);
                    }
                }
                changed
            }
            _ => false,
        }
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_mod(
        &mut self,
        fit_ids: &mut Vec<SolFitId>,
        sol_view: &SolView,
        item: &SolItem,
        modifier: &SolAttrMod,
    ) -> bool {
        // Maintain helper data containers
        self.by_affector.remove_entry(&modifier.affector_item_id, modifier);
        if matches!(item, SolItem::SwEffect(_)) {
            self.sw.remove(modifier);
        }
        if matches!(modifier.mod_type, SolModType::FleetBuff) {
            if let Some(fit_id) = item.get_fit_id() {
                self.fleet_fit.remove_entry(&fit_id, modifier);
            }
        }
        // Process simple modifiers which do not rely on any extra context
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Item => {
                    self.direct.remove_entry(&modifier.affector_item_id, modifier);
                    return true;
                }
                SolModDomain::Other => {
                    self.other.remove_entry(&modifier.affector_item_id, modifier);
                    return true;
                }
                _ => (),
            },
            _ => (),
        }
        match (modifier.mod_type, item) {
            // System-wide modifications affect all fits
            (SolModType::System | SolModType::Buff, SolItem::SwEffect(_)) => {
                fit_ids.clear();
                fit_ids.extend(sol_view.fits.iter_fit_ids());
                self.unapply_mod_from_fits(modifier, fit_ids)
            }
            // Fit-wide modifications affect only source fit itself
            (SolModType::System | SolModType::Buff, SolItem::FwEffect(fw_effect)) => {
                fit_ids.clear();
                fit_ids.push(fw_effect.fit_id);
                self.unapply_mod_from_fits(modifier, fit_ids)
            }
            // Local modifications are the same
            (SolModType::Local, _) => match item.get_fit_id() {
                Some(fit_id) => {
                    fit_ids.clear();
                    fit_ids.push(fit_id);
                    self.unapply_mod_from_fits(modifier, fit_ids)
                }
                None => false,
            },
            // Fleet modifications affect whole fleet, or just source fit itself, if fleet isn't set
            (SolModType::FleetBuff, SolItem::Module(module)) => {
                fill_fleet_fits(fit_ids, sol_view, module.fit_id);
                self.unapply_mod_from_fits(modifier, fit_ids)
            }
            // Various targetable effects affect only what they are target, depending on mod type
            (SolModType::System, SolItem::ProjEffect(proj_effect)) => {
                let mut changed = false;
                for tgt_item_id in proj_effect.tgts.iter_tgts() {
                    let tgt_item = sol_view.items.get_item(tgt_item_id).unwrap();
                    changed = changed | self.unapply_system_mod_from_item(modifier, tgt_item);
                }
                changed
            }
            (SolModType::Targeted, _) => {
                let mut changed = false;
                if let Some(tgt_item_ids) = item.iter_targets() {
                    for tgt_item_id in tgt_item_ids {
                        let tgt_item = sol_view.items.get_item(tgt_item_id).unwrap();
                        changed = changed | self.unapply_targeted_mod_from_item(modifier, tgt_item);
                    }
                }
                changed
            }
            (SolModType::Buff, _) => {
                let mut changed = false;
                if let Some(tgt_item_ids) = item.iter_targets() {
                    for tgt_item_id in tgt_item_ids {
                        let tgt_item = sol_view.items.get_item(tgt_item_id).unwrap();
                        changed = changed | self.unapply_buff_mod_from_item(modifier, tgt_item);
                    }
                }
                changed
            }
            _ => false,
        }
    }
    pub(in crate::sol::svc::svce_calc) fn add_mod_tgt(&mut self, modifier: SolAttrMod, tgt_item: &SolItem) -> bool {
        match modifier.mod_type {
            SolModType::System => self.apply_system_mod_to_item(modifier, tgt_item),
            SolModType::Targeted => self.apply_targeted_mod_to_item(modifier, tgt_item),
            SolModType::Buff => self.apply_buff_mod_to_item(modifier, tgt_item),
            _ => false,
        }
    }
    pub(in crate::sol::svc::svce_calc) fn rm_mod_tgt(&mut self, modifier: &SolAttrMod, tgt_item: &SolItem) -> bool {
        match modifier.mod_type {
            SolModType::System => self.unapply_system_mod_from_item(modifier, tgt_item),
            SolModType::Targeted => self.unapply_targeted_mod_from_item(modifier, tgt_item),
            SolModType::Buff => self.unapply_buff_mod_from_item(modifier, tgt_item),
            _ => false,
        }
    }
    // Private - fit methods
    fn apply_mod_to_fits(&mut self, modifier: SolAttrMod, fit_ids: &Vec<SolFitId>) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Everything => {
                    for fit_id in fit_ids.iter() {
                        self.buff_all.add_entry(*fit_id, modifier);
                    }
                    true
                }
                _ => match dom.try_into() {
                    Ok(loc) => {
                        for fit_id in fit_ids.iter() {
                            self.root.add_entry((*fit_id, loc), modifier);
                        }
                        true
                    }
                    _ => false,
                },
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolModDomain::Everything => {
                    for fit_id in fit_ids.iter() {
                        self.loc.add_entry((*fit_id, SolLocType::Ship), modifier);
                        self.loc.add_entry((*fit_id, SolLocType::Structure), modifier);
                    }
                    true
                }
                _ => match dom.try_into() {
                    Ok(loc) => {
                        for fit_id in fit_ids.iter() {
                            self.loc.add_entry((*fit_id, loc), modifier);
                        }
                        true
                    }
                    _ => false,
                },
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolModDomain::Everything => {
                    for fit_id in fit_ids.iter() {
                        self.loc_grp.add_entry((*fit_id, SolLocType::Ship, grp_id), modifier);
                        self.loc_grp
                            .add_entry((*fit_id, SolLocType::Structure, grp_id), modifier);
                    }
                    true
                }
                _ => match dom.try_into() {
                    Ok(loc) => {
                        for fit_id in fit_ids.iter() {
                            self.loc_grp.add_entry((*fit_id, loc, grp_id), modifier);
                        }
                        true
                    }
                    _ => false,
                },
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolModDomain::Everything => {
                    for fit_id in fit_ids.iter() {
                        self.loc_srq.add_entry((*fit_id, SolLocType::Ship, srq_id), modifier);
                        self.loc_srq
                            .add_entry((*fit_id, SolLocType::Structure, srq_id), modifier);
                    }
                    true
                }
                _ => match dom.try_into() {
                    Ok(loc) => {
                        for fit_id in fit_ids.iter() {
                            self.loc_srq.add_entry((*fit_id, loc, srq_id), modifier);
                        }
                        true
                    }
                    _ => false,
                },
            },
            SolAffecteeFilter::OwnSrq(srq_id) => {
                for fit_id in fit_ids.iter() {
                    self.own_srq.add_entry((*fit_id, srq_id), modifier);
                }
                true
            }
        }
    }
    fn unapply_mod_from_fits(&mut self, modifier: &SolAttrMod, fit_ids: &Vec<SolFitId>) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Everything => {
                    for fit_id in fit_ids.iter() {
                        self.buff_all.remove_entry(fit_id, &modifier);
                    }
                    true
                }
                _ => match dom.try_into() {
                    Ok(loc) => {
                        for fit_id in fit_ids.iter() {
                            self.root.remove_entry(&(*fit_id, loc), &modifier);
                        }
                        true
                    }
                    _ => false,
                },
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolModDomain::Everything => {
                    for fit_id in fit_ids.iter() {
                        self.loc.remove_entry(&(*fit_id, SolLocType::Ship), &modifier);
                        self.loc.remove_entry(&(*fit_id, SolLocType::Structure), &modifier);
                    }
                    true
                }
                _ => match dom.try_into() {
                    Ok(loc) => {
                        for fit_id in fit_ids.iter() {
                            self.loc.remove_entry(&(*fit_id, loc), &modifier);
                        }
                        true
                    }
                    _ => false,
                },
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolModDomain::Everything => {
                    for fit_id in fit_ids.iter() {
                        self.loc_grp
                            .remove_entry(&(*fit_id, SolLocType::Ship, grp_id), &modifier);
                        self.loc_grp
                            .remove_entry(&(*fit_id, SolLocType::Structure, grp_id), &modifier);
                    }
                    true
                }
                _ => match dom.try_into() {
                    Ok(loc) => {
                        for fit_id in fit_ids.iter() {
                            self.loc_grp.remove_entry(&(*fit_id, loc, grp_id), &modifier);
                        }
                        true
                    }
                    _ => false,
                },
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolModDomain::Everything => {
                    for fit_id in fit_ids.iter() {
                        self.loc_srq
                            .remove_entry(&(*fit_id, SolLocType::Ship, srq_id), &modifier);
                        self.loc_srq
                            .remove_entry(&(*fit_id, SolLocType::Structure, srq_id), &modifier);
                    }
                    true
                }
                _ => match dom.try_into() {
                    Ok(loc) => {
                        for fit_id in fit_ids.iter() {
                            self.loc_srq.remove_entry(&(*fit_id, loc, srq_id), &modifier);
                        }
                        true
                    }
                    _ => false,
                },
            },
            SolAffecteeFilter::OwnSrq(srq) => {
                for fit_id in fit_ids.iter() {
                    self.own_srq.remove_entry(&(*fit_id, srq), &modifier);
                }
                true
            }
        }
    }
    // Private - system-to-item methods
    fn apply_system_mod_to_item(&mut self, modifier: SolAttrMod, tgt_item: &SolItem) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.root.add_entry((tgt_ship.fit_id, SolLocType::Ship), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Structure => match tgt_item {
                    SolItem::Structure(tgt_struct) => {
                        self.root
                            .add_entry((tgt_struct.fit_id, SolLocType::Structure), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Char => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.root.add_entry((tgt_ship.fit_id, SolLocType::Character), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.root
                            .add_entry((tgt_struct.fit_id, SolLocType::Character), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc.add_entry((tgt_ship.fit_id, SolLocType::Ship), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Structure => match tgt_item {
                    SolItem::Structure(tgt_struct) => {
                        self.loc.add_entry((tgt_struct.fit_id, SolLocType::Structure), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Char => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc.add_entry((tgt_ship.fit_id, SolLocType::Character), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc.add_entry((tgt_struct.fit_id, SolLocType::Character), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_grp
                            .add_entry((tgt_ship.fit_id, SolLocType::Ship, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Structure => match tgt_item {
                    SolItem::Structure(tgt_struct) => {
                        self.loc_grp
                            .add_entry((tgt_struct.fit_id, SolLocType::Structure, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Char => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_grp
                            .add_entry((tgt_ship.fit_id, SolLocType::Character, grp_id), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc_grp
                            .add_entry((tgt_struct.fit_id, SolLocType::Character, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_srq
                            .add_entry((tgt_ship.fit_id, SolLocType::Ship, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Structure => match tgt_item {
                    SolItem::Structure(tgt_struct) => {
                        self.loc_srq
                            .add_entry((tgt_struct.fit_id, SolLocType::Structure, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Char => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_srq
                            .add_entry((tgt_ship.fit_id, SolLocType::Character, srq_id), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc_srq
                            .add_entry((tgt_struct.fit_id, SolLocType::Character, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match tgt_item {
                SolItem::Ship(tgt_ship) => {
                    self.own_srq.add_entry((tgt_ship.fit_id, srq_id), modifier);
                    true
                }
                SolItem::Structure(tgt_struct) => {
                    self.own_srq.add_entry((tgt_struct.fit_id, srq_id), modifier);
                    true
                }
                _ => false,
            },
        }
    }
    fn unapply_system_mod_from_item(&mut self, modifier: &SolAttrMod, tgt_item: &SolItem) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.root.remove_entry(&(tgt_ship.fit_id, SolLocType::Ship), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Structure => match tgt_item {
                    SolItem::Structure(tgt_struct) => {
                        self.root
                            .remove_entry(&(tgt_struct.fit_id, SolLocType::Structure), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Char => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.root
                            .remove_entry(&(tgt_ship.fit_id, SolLocType::Character), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.root
                            .remove_entry(&(tgt_struct.fit_id, SolLocType::Character), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc.remove_entry(&(tgt_ship.fit_id, SolLocType::Ship), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Structure => match tgt_item {
                    SolItem::Structure(tgt_struct) => {
                        self.loc
                            .remove_entry(&(tgt_struct.fit_id, SolLocType::Structure), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Char => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc
                            .remove_entry(&(tgt_ship.fit_id, SolLocType::Character), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc
                            .remove_entry(&(tgt_struct.fit_id, SolLocType::Character), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_grp
                            .remove_entry(&(tgt_ship.fit_id, SolLocType::Ship, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Structure => match tgt_item {
                    SolItem::Structure(tgt_struct) => {
                        self.loc_grp
                            .remove_entry(&(tgt_struct.fit_id, SolLocType::Structure, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Char => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_grp
                            .remove_entry(&(tgt_ship.fit_id, SolLocType::Character, grp_id), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc_grp
                            .remove_entry(&(tgt_struct.fit_id, SolLocType::Character, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_srq
                            .remove_entry(&(tgt_ship.fit_id, SolLocType::Ship, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Structure => match tgt_item {
                    SolItem::Structure(tgt_struct) => {
                        self.loc_srq
                            .remove_entry(&(tgt_struct.fit_id, SolLocType::Structure, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Char => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_srq
                            .remove_entry(&(tgt_ship.fit_id, SolLocType::Character, srq_id), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc_srq
                            .remove_entry(&(tgt_struct.fit_id, SolLocType::Character, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match tgt_item {
                SolItem::Ship(tgt_ship) => {
                    self.own_srq.remove_entry(&(tgt_ship.fit_id, srq_id), modifier);
                    true
                }
                SolItem::Structure(tgt_struct) => {
                    self.own_srq.remove_entry(&(tgt_struct.fit_id, srq_id), modifier);
                    true
                }
                _ => false,
            },
        }
    }
    // Private - targeted-to-item methods
    fn apply_targeted_mod_to_item(&mut self, modifier: SolAttrMod, tgt_item: &SolItem) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Target => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.root.add_entry((tgt_ship.fit_id, SolLocType::Ship), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.root
                            .add_entry((tgt_struct.fit_id, SolLocType::Structure), modifier);
                        true
                    }
                    _ => {
                        self.direct.add_entry(tgt_item.get_id(), modifier);
                        true
                    }
                },
                _ => false,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolModDomain::Target => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc.add_entry((tgt_ship.fit_id, SolLocType::Ship), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc.add_entry((tgt_struct.fit_id, SolLocType::Structure), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolModDomain::Target => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_grp
                            .add_entry((tgt_ship.fit_id, SolLocType::Ship, grp_id), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc_grp
                            .add_entry((tgt_struct.fit_id, SolLocType::Structure, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolModDomain::Target => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_srq
                            .add_entry((tgt_ship.fit_id, SolLocType::Ship, srq_id), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc_srq
                            .add_entry((tgt_struct.fit_id, SolLocType::Structure, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match tgt_item {
                SolItem::Ship(tgt_ship) => {
                    self.own_srq.add_entry((tgt_ship.fit_id, srq_id), modifier);
                    true
                }
                SolItem::Structure(tgt_struct) => {
                    self.own_srq.add_entry((tgt_struct.fit_id, srq_id), modifier);
                    true
                }
                _ => false,
            },
        }
    }
    fn unapply_targeted_mod_from_item(&mut self, modifier: &SolAttrMod, tgt_item: &SolItem) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Target => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.root.remove_entry(&(tgt_ship.fit_id, SolLocType::Ship), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.root
                            .remove_entry(&(tgt_struct.fit_id, SolLocType::Structure), modifier);
                        true
                    }
                    _ => {
                        self.direct.remove_entry(&tgt_item.get_id(), modifier);
                        true
                    }
                },
                _ => false,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolModDomain::Target => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc.remove_entry(&(tgt_ship.fit_id, SolLocType::Ship), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc
                            .remove_entry(&(tgt_struct.fit_id, SolLocType::Structure), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolModDomain::Target => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_grp
                            .remove_entry(&(tgt_ship.fit_id, SolLocType::Ship, grp_id), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc_grp
                            .remove_entry(&(tgt_struct.fit_id, SolLocType::Structure, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolModDomain::Target => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_srq
                            .remove_entry(&(tgt_ship.fit_id, SolLocType::Ship, srq_id), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc_srq
                            .remove_entry(&(tgt_struct.fit_id, SolLocType::Structure, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match tgt_item {
                SolItem::Ship(tgt_ship) => {
                    self.own_srq.remove_entry(&(tgt_ship.fit_id, srq_id), modifier);
                    true
                }
                SolItem::Structure(tgt_struct) => {
                    self.own_srq.remove_entry(&(tgt_struct.fit_id, srq_id), modifier);
                    true
                }
                _ => false,
            },
        }
    }
    // Private - buff-to-item methods
    fn apply_buff_mod_to_item(&mut self, modifier: SolAttrMod, tgt_item: &SolItem) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Everything => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.root.add_entry((tgt_ship.fit_id, SolLocType::Ship), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.root
                            .add_entry((tgt_struct.fit_id, SolLocType::Structure), modifier);
                        true
                    }
                    _ if tgt_item.is_buff_modifiable() => {
                        self.direct.add_entry(tgt_item.get_id(), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.root.add_entry((tgt_ship.fit_id, SolLocType::Ship), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolModDomain::Everything => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc.add_entry((tgt_ship.fit_id, SolLocType::Ship), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc.add_entry((tgt_struct.fit_id, SolLocType::Structure), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc.add_entry((tgt_ship.fit_id, SolLocType::Ship), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolModDomain::Everything => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_grp
                            .add_entry((tgt_ship.fit_id, SolLocType::Ship, grp_id), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc_grp
                            .add_entry((tgt_struct.fit_id, SolLocType::Structure, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_grp
                            .add_entry((tgt_ship.fit_id, SolLocType::Ship, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolModDomain::Everything => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_srq
                            .add_entry((tgt_ship.fit_id, SolLocType::Ship, srq_id), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc_srq
                            .add_entry((tgt_struct.fit_id, SolLocType::Structure, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_srq
                            .add_entry((tgt_ship.fit_id, SolLocType::Ship, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    }
    fn unapply_buff_mod_from_item(&mut self, modifier: &SolAttrMod, tgt_item: &SolItem) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Everything => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.root.remove_entry(&(tgt_ship.fit_id, SolLocType::Ship), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.root
                            .remove_entry(&(tgt_struct.fit_id, SolLocType::Structure), modifier);
                        true
                    }
                    _ if tgt_item.is_buff_modifiable() => {
                        self.direct.remove_entry(&tgt_item.get_id(), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.root.remove_entry(&(tgt_ship.fit_id, SolLocType::Ship), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolModDomain::Everything => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc.remove_entry(&(tgt_ship.fit_id, SolLocType::Ship), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc
                            .remove_entry(&(tgt_struct.fit_id, SolLocType::Structure), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc.remove_entry(&(tgt_ship.fit_id, SolLocType::Ship), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolModDomain::Everything => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_grp
                            .remove_entry(&(tgt_ship.fit_id, SolLocType::Ship, grp_id), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc_grp
                            .remove_entry(&(tgt_struct.fit_id, SolLocType::Structure, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_grp
                            .remove_entry(&(tgt_ship.fit_id, SolLocType::Ship, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolModDomain::Everything => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_srq
                            .remove_entry(&(tgt_ship.fit_id, SolLocType::Ship, srq_id), modifier);
                        true
                    }
                    SolItem::Structure(tgt_struct) => {
                        self.loc_srq
                            .remove_entry(&(tgt_struct.fit_id, SolLocType::Structure, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        self.loc_srq
                            .remove_entry(&(tgt_ship.fit_id, SolLocType::Ship, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    }
    // Private - misc methods
    fn get_fleet_updates(&self, fleet: &SolFleet, fit_id: &SolFitId) -> SolFleetUpdates {
        let mut updates = SolFleetUpdates::new();
        updates.outgoing.extend(self.fleet_fit.get(fit_id));
        for fleet_fit_id in fleet.iter_fits() {
            if fleet_fit_id == fit_id {
                continue;
            }
            updates.incoming.extend(self.fleet_fit.get(fleet_fit_id));
        }
        updates
    }
}

fn compare_loc_dom(loc: SolLocType, dom: SolModDomain) -> bool {
    match (loc, dom) {
        (SolLocType::Ship, SolModDomain::Everything) => true,
        (SolLocType::Ship, SolModDomain::Ship) => true,
        (SolLocType::Structure, SolModDomain::Everything) => true,
        (SolLocType::Structure, SolModDomain::Structure) => true,
        (SolLocType::Character, SolModDomain::Char) => true,
        _ => false,
    }
}

fn filter_and_extend<K: Eq + Hash>(
    vec: &mut Vec<SolAttrMod>,
    storage: &StMapSetL1<K, SolAttrMod>,
    key: &K,
    attr_id: &EAttrId,
) {
    vec.extend(storage.get(key).filter(|v| &v.affectee_attr_id == attr_id).map(|v| *v))
}

fn fill_fleet_fits(fit_ids: &mut Vec<SolFitId>, sol_view: &SolView, fit_id: SolFitId) {
    let src_fit = sol_view.fits.get_fit(&fit_id).unwrap();
    fit_ids.clear();
    match src_fit.fleet {
        Some(fleet_id) => {
            let fleet = sol_view.fleets.get_fleet(&fleet_id).unwrap();
            fit_ids.extend(fleet.iter_fits());
        }
        None => fit_ids.push(fit_id),
    }
}
