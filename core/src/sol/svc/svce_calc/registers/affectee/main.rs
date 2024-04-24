use std::convert::TryInto;

use crate::{
    defs::{EItemGrpId, EItemId, SolFitId, SolItemId},
    sol::{
        fit::SolFit,
        item::SolItem,
        svc::svce_calc::{SolAffecteeFilter, SolAttrMod, SolLocType, SolModDomain, SolModType},
        SolView,
    },
    util::{extend_vec_from_map_set_l1, StMapSetL1},
};

use super::PotentialLocations;

pub(in crate::sol::svc::svce_calc) struct SolAffecteeRegister {
    // Items which are holders of a location type (like char, ship)
    // Map<(affectee fit ID, affectee location type), affectee item IDs>
    pub(super) root: StMapSetL1<(SolFitId, SolLocType), SolItemId>,
    // Items belonging to certain fit and location type (e.g. char's implants, ship's modules)
    // Map<(affectee fit ID, affectee location type), affectee item IDs>
    pub(super) loc: StMapSetL1<(SolFitId, SolLocType), SolItemId>,
    // Items belonging to certain fit, location type and group
    // Map<(affectee fit ID, affectee location type, affectee group ID), affectee item IDs>
    pub(super) loc_grp: StMapSetL1<(SolFitId, SolLocType, EItemGrpId), SolItemId>,
    // Items belonging to certain fit and location type, and having certain skill requirement
    // Map<(affectee fit ID, affectee location type, affectee skillreq type ID), affectee item IDs>
    pub(super) loc_srq: StMapSetL1<(SolFitId, SolLocType, EItemId), SolItemId>,
    // Owner-modifiable items which belong to certain fit and have certain skill requirement
    // Map<(affectee fit ID, affectee skillreq type ID), affectee item IDs>
    pub(super) own_srq: StMapSetL1<(SolFitId, EItemId), SolItemId>,
    // Everything-buff-modifiable items which belong to certain fit
    // Map<affectee fit ID, affectee item IDs>
    pub(super) buff_all: StMapSetL1<SolFitId, SolItemId>,
}
impl SolAffecteeRegister {
    pub(in crate::sol::svc::svce_calc) fn new() -> Self {
        Self {
            root: StMapSetL1::new(),
            loc: StMapSetL1::new(),
            loc_grp: StMapSetL1::new(),
            loc_srq: StMapSetL1::new(),
            own_srq: StMapSetL1::new(),
            buff_all: StMapSetL1::new(),
        }
    }
    // Query methods
    pub(in crate::sol::svc::svce_calc) fn fill_affectees(
        &self,
        affectees: &mut Vec<SolItemId>,
        sol_view: &SolView,
        item: &SolItem,
        modifier: &SolAttrMod,
    ) {
        // Those modifiers work the same regardless of broader context. They just need an item which
        // carries them.
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Item => {
                    affectees.push(modifier.affector_item_id);
                    return;
                }
                SolModDomain::Other => {
                    if let Some(other_item_id) = item.get_other() {
                        affectees.push(other_item_id);
                    }
                    return;
                }
                _ => (),
            },
            _ => (),
        }
        match (modifier.mod_type, item) {
            // System-wide modifications affect all fits
            (SolModType::System | SolModType::Buff, SolItem::SwEffect(_)) => {
                for fit in sol_view.fits.iter_fits() {
                    self.fill_affectees_for_fit(affectees, modifier, fit);
                }
            }
            // Fit-wide modifications affect only source fit itself
            (SolModType::System | SolModType::Buff, SolItem::FwEffect(fw_effect)) => {
                let fit = sol_view.fits.get_fit(&fw_effect.fit_id).unwrap();
                self.fill_affectees_for_fit(affectees, modifier, fit);
            }
            // Local modifications are the same
            (SolModType::Local, _) => {
                if let Some(fit_id) = item.get_fit_id() {
                    let fit = sol_view.fits.get_fit(&fit_id).unwrap();
                    self.fill_affectees_for_fit(affectees, modifier, fit);
                }
            }
            // Fleet modifications affect whole fleet, or just source fit itself, if fleet isn't set
            (SolModType::FleetBuff, SolItem::Module(module)) => {
                let src_fit = sol_view.fits.get_fit(&module.fit_id).unwrap();
                match src_fit.fleet {
                    Some(fleet_id) => {
                        let fleet = sol_view.fleets.get_fleet(&fleet_id).unwrap();
                        for dst_fit in fleet.iter_fits().map(|v| sol_view.fits.get_fit(v).unwrap()) {
                            self.fill_affectees_for_fit(affectees, modifier, dst_fit);
                        }
                    }
                    None => self.fill_affectees_for_fit(affectees, modifier, src_fit),
                }
            }
            // Various targetable effects affect only what they are target, depending on mod type
            (SolModType::System, SolItem::ProjEffect(proj_effect)) => {
                for tgt_item_id in proj_effect.tgts.iter_tgts() {
                    let tgt_item = sol_view.items.get_item(tgt_item_id).unwrap();
                    self.fill_tgt_for_system_mod(affectees, sol_view, modifier, tgt_item);
                }
            }
            (SolModType::Targeted, _) => {
                if let Some(tgt_item_ids) = item.iter_targets() {
                    for tgt_item_id in tgt_item_ids {
                        let tgt_item = sol_view.items.get_item(tgt_item_id).unwrap();
                        self.fill_tgt_for_targeted_mod(affectees, modifier, tgt_item);
                    }
                }
            }
            (SolModType::Buff, _) => {
                if let Some(tgt_item_ids) = item.iter_targets() {
                    for tgt_item_id in tgt_item_ids {
                        let tgt_item = sol_view.items.get_item(tgt_item_id).unwrap();
                        self.fill_tgt_for_buff_mod(affectees, modifier, tgt_item);
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::sol::svc::svce_calc) fn fill_affectees_for_fit(
        &self,
        affectees: &mut Vec<SolItemId>,
        modifier: &SolAttrMod,
        fit: &SolFit,
    ) {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Everything => extend_vec_from_map_set_l1(affectees, &self.buff_all, &fit.id),
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        if check_domain_owner(dom, fit) {
                            extend_vec_from_map_set_l1(affectees, &self.root, &(fit.id, loc));
                        }
                    }
                }
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolModDomain::Everything => {
                    if check_domain_owner(SolModDomain::Ship, fit) {
                        extend_vec_from_map_set_l1(affectees, &self.loc, &(fit.id, SolLocType::Ship));
                    }
                    if check_domain_owner(SolModDomain::Structure, fit) {
                        extend_vec_from_map_set_l1(affectees, &self.loc, &(fit.id, SolLocType::Structure));
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        if check_domain_owner(dom, fit) {
                            extend_vec_from_map_set_l1(affectees, &self.loc, &(fit.id, loc));
                        }
                    }
                }
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolModDomain::Everything => {
                    if check_domain_owner(SolModDomain::Ship, fit) {
                        extend_vec_from_map_set_l1(affectees, &self.loc_grp, &(fit.id, SolLocType::Ship, grp_id));
                    }
                    if check_domain_owner(SolModDomain::Structure, fit) {
                        extend_vec_from_map_set_l1(affectees, &self.loc_grp, &(fit.id, SolLocType::Structure, grp_id));
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        if check_domain_owner(dom, fit) {
                            extend_vec_from_map_set_l1(affectees, &self.loc_grp, &(fit.id, loc, grp_id));
                        }
                    }
                }
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolModDomain::Everything => {
                    if check_domain_owner(SolModDomain::Ship, fit) {
                        extend_vec_from_map_set_l1(affectees, &self.loc_srq, &(fit.id, SolLocType::Ship, srq_id));
                    }
                    if check_domain_owner(SolModDomain::Structure, fit) {
                        extend_vec_from_map_set_l1(affectees, &self.loc_srq, &(fit.id, SolLocType::Structure, srq_id));
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        if check_domain_owner(dom, fit) {
                            extend_vec_from_map_set_l1(affectees, &self.loc_srq, &(fit.id, loc, srq_id));
                        }
                    }
                }
            },
            SolAffecteeFilter::OwnSrq(srq_id) => {
                extend_vec_from_map_set_l1(affectees, &self.own_srq, &(fit.id, srq_id));
            }
        }
    }
    pub(in crate::sol::svc::svce_calc) fn fill_affectees_for_tgt_item(
        &self,
        affectees: &mut Vec<SolItemId>,
        sol_view: &SolView,
        modifier: &SolAttrMod,
        tgt_item: &SolItem,
    ) {
        match modifier.mod_type {
            SolModType::System => self.fill_tgt_for_system_mod(affectees, sol_view, modifier, tgt_item),
            SolModType::Targeted => self.fill_tgt_for_targeted_mod(affectees, modifier, tgt_item),
            SolModType::Buff => self.fill_tgt_for_buff_mod(affectees, modifier, tgt_item),
            _ => (),
        }
    }
    // Modification methods
    pub(in crate::sol::svc::svce_calc) fn reg_affectee(&mut self, sol_view: &SolView, item: &SolItem) {
        let item_id = item.get_id();
        let fit_opt = item.get_fit_id().map(|v| sol_view.fits.get_fit(&v).ok()).flatten();
        let root_loc_opt = item.get_root_loc_type();
        let grp_id_opt = item.get_group_id().ok();
        let srqs_opt = item.get_skill_reqs().ok();
        if let (Some(fit), Some(root_loc)) = (fit_opt, root_loc_opt) {
            self.root.add_entry((fit.id, root_loc), item_id);
        }
        if let Some(fit) = fit_opt {
            for loc_type in PotentialLocations::new(item) {
                self.loc.add_entry((fit.id, loc_type), item_id);
            }
        }
        if let (Some(fit), Some(grp_id)) = (fit_opt, grp_id_opt) {
            for loc_type in PotentialLocations::new(item) {
                self.loc_grp.add_entry((fit.id, loc_type, grp_id), item_id);
            }
        }
        if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
            for loc_type in PotentialLocations::new(item) {
                for srq_id in srqs.keys() {
                    self.loc_srq.add_entry((fit.id, loc_type, *srq_id), item_id);
                }
            }
        }
        if item.is_owner_modifiable() {
            if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
                for skill_a_item_id in srqs.keys() {
                    self.own_srq.add_entry((fit.id, *skill_a_item_id), item_id);
                }
            }
        }
        if item.is_buff_modifiable() {
            if let Some(fit) = fit_opt {
                self.buff_all.add_entry(fit.id, item_id);
            }
        }
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_affectee(&mut self, sol_view: &SolView, item: &SolItem) {
        let item_id = item.get_id();
        let fit_opt = item.get_fit_id().map(|v| sol_view.fits.get_fit(&v).ok()).flatten();
        let root_loc_opt = item.get_root_loc_type();
        let grp_id_opt = item.get_group_id().ok();
        let srqs_opt = item.get_skill_reqs().ok();
        if let (Some(fit), Some(root_loc)) = (fit_opt, root_loc_opt) {
            self.root.remove_entry(&(fit.id, root_loc), &item_id);
        }
        if let Some(fit) = fit_opt {
            for loc_type in PotentialLocations::new(item) {
                self.loc.remove_entry(&(fit.id, loc_type), &item_id);
            }
        }
        if let (Some(fit), Some(grp_id)) = (fit_opt, grp_id_opt) {
            for loc_type in PotentialLocations::new(item) {
                self.loc_grp.remove_entry(&(fit.id, loc_type, grp_id), &item_id);
            }
        }
        if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
            for loc_type in PotentialLocations::new(item) {
                for srq_id in srqs.keys() {
                    self.loc_srq.remove_entry(&(fit.id, loc_type, *srq_id), &item_id);
                }
            }
        }
        if item.is_owner_modifiable() {
            if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
                for srq_id in srqs.keys() {
                    self.own_srq.remove_entry(&(fit.id, *srq_id), &item_id);
                }
            }
        }
        if item.is_buff_modifiable() {
            if let Some(fit) = fit_opt {
                self.buff_all.remove_entry(&fit.id, &item_id);
            }
        }
    }
    // Private methods
    fn fill_tgt_for_system_mod(
        &self,
        affectees: &mut Vec<SolItemId>,
        sol_view: &SolView,
        modifier: &SolAttrMod,
        tgt_item: &SolItem,
    ) {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Ship if matches!(tgt_item, SolItem::Ship(_)) => affectees.push(tgt_item.get_id()),
                SolModDomain::Structure if matches!(tgt_item, SolItem::Structure(_)) => {
                    affectees.push(tgt_item.get_id())
                }
                SolModDomain::Char => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        if let Some(char_id) = get_fit_character(sol_view, &tgt_ship.fit_id) {
                            affectees.push(char_id);
                        }
                    }
                    SolItem::Structure(tgt_struct) => {
                        if let Some(char_id) = get_fit_character(sol_view, &tgt_struct.fit_id) {
                            affectees.push(char_id);
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        extend_vec_from_map_set_l1(affectees, &self.loc, &(tgt_ship.fit_id, SolLocType::Ship))
                    }
                    _ => (),
                },
                SolModDomain::Structure => match tgt_item {
                    SolItem::Structure(tgt_struct) => {
                        extend_vec_from_map_set_l1(affectees, &self.loc, &(tgt_struct.fit_id, SolLocType::Structure))
                    }
                    _ => (),
                },
                SolModDomain::Char => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        extend_vec_from_map_set_l1(affectees, &self.loc, &(tgt_ship.fit_id, SolLocType::Character))
                    }
                    SolItem::Structure(tgt_struct) => {
                        extend_vec_from_map_set_l1(affectees, &self.loc, &(tgt_struct.fit_id, SolLocType::Character))
                    }
                    _ => (),
                },
                _ => (),
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_grp,
                        &(tgt_ship.fit_id, SolLocType::Ship, grp_id),
                    ),
                    _ => (),
                },
                SolModDomain::Structure => match tgt_item {
                    SolItem::Structure(tgt_struct) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_grp,
                        &(tgt_struct.fit_id, SolLocType::Structure, grp_id),
                    ),
                    _ => (),
                },
                SolModDomain::Char => match tgt_item {
                    SolItem::Ship(tgt_ship) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_grp,
                        &(tgt_ship.fit_id, SolLocType::Character, grp_id),
                    ),
                    SolItem::Structure(tgt_struct) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_grp,
                        &(tgt_struct.fit_id, SolLocType::Character, grp_id),
                    ),
                    _ => (),
                },
                _ => (),
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_srq,
                        &(tgt_ship.fit_id, SolLocType::Ship, srq_id),
                    ),
                    _ => (),
                },
                SolModDomain::Structure => match tgt_item {
                    SolItem::Structure(tgt_struct) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_srq,
                        &(tgt_struct.fit_id, SolLocType::Structure, srq_id),
                    ),
                    _ => (),
                },
                SolModDomain::Char => match tgt_item {
                    SolItem::Ship(tgt_ship) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_srq,
                        &(tgt_ship.fit_id, SolLocType::Character, srq_id),
                    ),
                    SolItem::Structure(tgt_struct) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_srq,
                        &(tgt_struct.fit_id, SolLocType::Character, srq_id),
                    ),
                    _ => (),
                },
                _ => (),
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match tgt_item {
                SolItem::Ship(tgt_ship) => {
                    extend_vec_from_map_set_l1(affectees, &self.own_srq, &(tgt_ship.fit_id, srq_id))
                }
                SolItem::Structure(tgt_struct) => {
                    extend_vec_from_map_set_l1(affectees, &self.own_srq, &(tgt_struct.fit_id, srq_id))
                }
                _ => (),
            },
        }
    }
    fn fill_tgt_for_targeted_mod(&self, affectees: &mut Vec<SolItemId>, modifier: &SolAttrMod, tgt_item: &SolItem) {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => {
                if matches!(dom, SolModDomain::Target) {
                    affectees.push(tgt_item.get_id())
                }
            }
            SolAffecteeFilter::Loc(dom) => {
                if matches!(dom, SolModDomain::Target) {
                    match tgt_item {
                        SolItem::Ship(tgt_ship) => {
                            extend_vec_from_map_set_l1(affectees, &self.loc, &(tgt_ship.fit_id, SolLocType::Ship))
                        }
                        SolItem::Structure(tgt_struct) => extend_vec_from_map_set_l1(
                            affectees,
                            &self.loc,
                            &(tgt_struct.fit_id, SolLocType::Structure),
                        ),
                        _ => (),
                    }
                }
            }
            SolAffecteeFilter::LocGrp(dom, grp_id) => {
                if matches!(dom, SolModDomain::Target) {
                    match tgt_item {
                        SolItem::Ship(tgt_ship) => extend_vec_from_map_set_l1(
                            affectees,
                            &self.loc_grp,
                            &(tgt_ship.fit_id, SolLocType::Ship, grp_id),
                        ),
                        SolItem::Structure(tgt_struct) => extend_vec_from_map_set_l1(
                            affectees,
                            &self.loc_grp,
                            &(tgt_struct.fit_id, SolLocType::Structure, grp_id),
                        ),
                        _ => (),
                    }
                }
            }
            SolAffecteeFilter::LocSrq(dom, srq_id) => {
                if matches!(dom, SolModDomain::Target) {
                    match tgt_item {
                        SolItem::Ship(tgt_ship) => extend_vec_from_map_set_l1(
                            affectees,
                            &self.loc_srq,
                            &(tgt_ship.fit_id, SolLocType::Ship, srq_id),
                        ),
                        SolItem::Structure(tgt_struct) => extend_vec_from_map_set_l1(
                            affectees,
                            &self.loc_srq,
                            &(tgt_struct.fit_id, SolLocType::Structure, srq_id),
                        ),
                        _ => (),
                    }
                }
            }
            SolAffecteeFilter::OwnSrq(srq_id) => match tgt_item {
                SolItem::Ship(tgt_ship) => {
                    extend_vec_from_map_set_l1(affectees, &self.own_srq, &(tgt_ship.fit_id, srq_id))
                }
                SolItem::Structure(tgt_struct) => {
                    extend_vec_from_map_set_l1(affectees, &self.own_srq, &(tgt_struct.fit_id, srq_id))
                }
                _ => (),
            },
        }
    }
    fn fill_tgt_for_buff_mod(&self, affectees: &mut Vec<SolItemId>, modifier: &SolAttrMod, tgt_item: &SolItem) {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Everything => {
                    if tgt_item.is_buff_modifiable() {
                        affectees.push(tgt_item.get_id())
                    }
                }
                SolModDomain::Ship if matches!(tgt_item, SolItem::Ship(_)) => affectees.push(tgt_item.get_id()),
                _ => (),
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolModDomain::Everything => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        extend_vec_from_map_set_l1(affectees, &self.loc, &(tgt_ship.fit_id, SolLocType::Ship))
                    }
                    SolItem::Structure(tgt_struct) => {
                        extend_vec_from_map_set_l1(affectees, &self.loc, &(tgt_struct.fit_id, SolLocType::Structure))
                    }
                    _ => (),
                },
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => {
                        extend_vec_from_map_set_l1(affectees, &self.loc, &(tgt_ship.fit_id, SolLocType::Ship))
                    }
                    _ => (),
                },
                _ => (),
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolModDomain::Everything => match tgt_item {
                    SolItem::Ship(tgt_ship) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_grp,
                        &(tgt_ship.fit_id, SolLocType::Ship, grp_id),
                    ),
                    SolItem::Structure(tgt_struct) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_grp,
                        &(tgt_struct.fit_id, SolLocType::Structure, grp_id),
                    ),
                    _ => (),
                },
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_grp,
                        &(tgt_ship.fit_id, SolLocType::Ship, grp_id),
                    ),
                    _ => (),
                },
                _ => (),
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolModDomain::Everything => match tgt_item {
                    SolItem::Ship(tgt_ship) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_srq,
                        &(tgt_ship.fit_id, SolLocType::Ship, srq_id),
                    ),
                    SolItem::Structure(tgt_struct) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_srq,
                        &(tgt_struct.fit_id, SolLocType::Structure, srq_id),
                    ),
                    _ => (),
                },
                SolModDomain::Ship => match tgt_item {
                    SolItem::Ship(tgt_ship) => extend_vec_from_map_set_l1(
                        affectees,
                        &self.loc_srq,
                        &(tgt_ship.fit_id, SolLocType::Ship, srq_id),
                    ),
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }
}

fn get_fit_character(sol_view: &SolView, fit_id: &SolFitId) -> Option<SolItemId> {
    sol_view.fits.get_fit(fit_id).ok().map(|v| v.character).flatten()
}

fn check_domain_owner(dom: SolModDomain, fit: &SolFit) -> bool {
    match dom {
        SolModDomain::Char => fit.character.is_some(),
        SolModDomain::Ship => fit.ship.is_some(),
        SolModDomain::Structure => fit.structure.is_some(),
        _ => false,
    }
}
