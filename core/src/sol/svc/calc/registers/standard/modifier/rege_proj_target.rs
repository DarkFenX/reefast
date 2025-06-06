use super::{add_ctx_modifier, remove_ctx_modifier};
use crate::sol::{
    ItemKey,
    svc::calc::{AffecteeFilter, CtxModifier, Location, LocationKind, RawModifier, registers::StandardRegister},
    uad::item::{ShipKind, UadItem},
};

impl StandardRegister {
    pub(super) fn proj_target_mod(
        &mut self,
        raw_modifier: RawModifier,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
    ) -> Option<CtxModifier> {
        self.process_target_mod(raw_modifier, projectee_item_key, projectee_item, true)
    }
    pub(super) fn query_target_mod(
        &mut self,
        raw_modifier: RawModifier,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
    ) -> Option<CtxModifier> {
        self.process_target_mod(raw_modifier, projectee_item_key, projectee_item, false)
    }
    fn process_target_mod(
        &mut self,
        raw_modifier: RawModifier,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
        register: bool,
    ) -> Option<CtxModifier> {
        match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Target => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                    add_ctx_modifier(
                        &mut self.cmods_direct,
                        projectee_item_key,
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    self.rmods_proj_active.add_entry(projectee_item_key, raw_modifier);
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::Loc(loc) => match loc {
                Location::Target => match projectee_item {
                    UadItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            if register {
                                add_ctx_modifier(
                                    &mut self.cmods_loc,
                                    (projectee_ship.get_fit_key(), LocationKind::Ship),
                                    ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                                self.rmods_proj_active.add_entry(projectee_item_key, raw_modifier);
                            }
                            Some(ctx_modifier)
                        }
                        ShipKind::Structure => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            if register {
                                add_ctx_modifier(
                                    &mut self.cmods_loc,
                                    (projectee_ship.get_fit_key(), LocationKind::Structure),
                                    ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                                self.rmods_proj_active.add_entry(projectee_item_key, raw_modifier);
                            }
                            Some(ctx_modifier)
                        }
                        _ => self.reg_inactive_proj_rmod(raw_modifier, projectee_item_key, register),
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc {
                Location::Target => match projectee_item {
                    UadItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            if register {
                                add_ctx_modifier(
                                    &mut self.cmods_loc_grp,
                                    (projectee_ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                                    ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                                self.rmods_proj_active.add_entry(projectee_item_key, raw_modifier);
                            }
                            Some(ctx_modifier)
                        }
                        ShipKind::Structure => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            if register {
                                add_ctx_modifier(
                                    &mut self.cmods_loc_grp,
                                    (projectee_ship.get_fit_key(), LocationKind::Structure, a_item_grp_id),
                                    ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                                self.rmods_proj_active.add_entry(projectee_item_key, raw_modifier);
                            }
                            Some(ctx_modifier)
                        }
                        _ => self.reg_inactive_proj_rmod(raw_modifier, projectee_item_key, register),
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc {
                Location::Target => match projectee_item {
                    UadItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            if register {
                                add_ctx_modifier(
                                    &mut self.cmods_loc_srq,
                                    (projectee_ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                                    ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                                self.rmods_proj_active.add_entry(projectee_item_key, raw_modifier);
                            }
                            Some(ctx_modifier)
                        }
                        ShipKind::Structure => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            if register {
                                add_ctx_modifier(
                                    &mut self.cmods_loc_srq,
                                    (projectee_ship.get_fit_key(), LocationKind::Structure, srq_a_item_id),
                                    ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                                self.rmods_proj_active.add_entry(projectee_item_key, raw_modifier);
                            }
                            Some(ctx_modifier)
                        }
                        _ => self.reg_inactive_proj_rmod(raw_modifier, projectee_item_key, register),
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => match projectee_item {
                UadItem::Ship(projectee_ship) => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                    if register {
                        add_ctx_modifier(
                            &mut self.cmods_own_srq,
                            (projectee_ship.get_fit_key(), srq_a_item_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        self.rmods_proj_active.add_entry(projectee_item_key, raw_modifier);
                    }
                    Some(ctx_modifier)
                }
                _ => None,
            },
        }
    }
    pub(super) fn unproj_target_mod(
        &mut self,
        raw_modifier: RawModifier,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
    ) -> Option<CtxModifier> {
        match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Target => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                    remove_ctx_modifier(
                        &mut self.cmods_direct,
                        &projectee_item_key,
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    self.rmods_proj_active.remove_entry(&projectee_item_key, &raw_modifier);
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::Loc(loc) => match loc {
                Location::Target => match projectee_item {
                    UadItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc,
                                &(projectee_ship.get_fit_key(), LocationKind::Ship),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            self.rmods_proj_active.remove_entry(&projectee_item_key, &raw_modifier);
                            Some(ctx_modifier)
                        }
                        ShipKind::Structure => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc,
                                &(projectee_ship.get_fit_key(), LocationKind::Structure),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            self.rmods_proj_active.remove_entry(&projectee_item_key, &raw_modifier);
                            Some(ctx_modifier)
                        }
                        _ => self.unreg_inactive_proj_rmod(&raw_modifier, &projectee_item_key),
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc {
                Location::Target => match projectee_item {
                    UadItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc_grp,
                                &(projectee_ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            self.rmods_proj_active.remove_entry(&projectee_item_key, &raw_modifier);
                            Some(ctx_modifier)
                        }
                        ShipKind::Structure => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc_grp,
                                &(projectee_ship.get_fit_key(), LocationKind::Structure, a_item_grp_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            self.rmods_proj_active.remove_entry(&projectee_item_key, &raw_modifier);
                            Some(ctx_modifier)
                        }
                        _ => self.unreg_inactive_proj_rmod(&raw_modifier, &projectee_item_key),
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc {
                Location::Target => match projectee_item {
                    UadItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc_srq,
                                &(projectee_ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            self.rmods_proj_active.remove_entry(&projectee_item_key, &raw_modifier);
                            Some(ctx_modifier)
                        }
                        ShipKind::Structure => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc_srq,
                                &(projectee_ship.get_fit_key(), LocationKind::Structure, srq_a_item_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            self.rmods_proj_active.remove_entry(&projectee_item_key, &raw_modifier);
                            Some(ctx_modifier)
                        }
                        _ => self.unreg_inactive_proj_rmod(&raw_modifier, &projectee_item_key),
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => match projectee_item {
                UadItem::Ship(projectee_ship) => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                    remove_ctx_modifier(
                        &mut self.cmods_own_srq,
                        &(projectee_ship.get_fit_key(), srq_a_item_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    self.rmods_proj_active.remove_entry(&projectee_item_key, &raw_modifier);
                    Some(ctx_modifier)
                }
                _ => None,
            },
        }
    }
    pub(super) fn reg_loc_root_for_proj_target(
        &mut self,
        raw_modifier: RawModifier,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
    ) {
        self.process_target_mod(raw_modifier, projectee_item_key, projectee_item, true);
    }

    pub(super) fn unreg_loc_root_for_proj_target(
        &mut self,
        raw_modifier: RawModifier,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
    ) {
        match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => {
                if let Location::Target = loc {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                    remove_ctx_modifier(
                        &mut self.cmods_direct,
                        &projectee_item_key,
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    self.rmods_proj_inactive.add_entry(projectee_item_key, raw_modifier);
                }
            }
            AffecteeFilter::Loc(loc) => {
                if let Location::Target = loc
                    && let UadItem::Ship(projectee_ship) = projectee_item
                {
                    match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc,
                                &(projectee_ship.get_fit_key(), LocationKind::Ship),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            self.rmods_proj_inactive.add_entry(projectee_item_key, raw_modifier);
                        }
                        ShipKind::Structure => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc,
                                &(projectee_ship.get_fit_key(), LocationKind::Structure),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            self.rmods_proj_inactive.add_entry(projectee_item_key, raw_modifier);
                        }
                        _ => (),
                    }
                }
            }
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => {
                if let Location::Target = loc
                    && let UadItem::Ship(projectee_ship) = projectee_item
                {
                    match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc_grp,
                                &(projectee_ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            self.rmods_proj_inactive.add_entry(projectee_item_key, raw_modifier);
                        }
                        ShipKind::Structure => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc_grp,
                                &(projectee_ship.get_fit_key(), LocationKind::Structure, a_item_grp_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            self.rmods_proj_inactive.add_entry(projectee_item_key, raw_modifier);
                        }
                        _ => (),
                    }
                }
            }
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => {
                if let Location::Target = loc
                    && let UadItem::Ship(projectee_ship) = projectee_item
                {
                    match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc_srq,
                                &(projectee_ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            self.rmods_proj_inactive.add_entry(projectee_item_key, raw_modifier);
                        }
                        ShipKind::Structure => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc_srq,
                                &(projectee_ship.get_fit_key(), LocationKind::Structure, srq_a_item_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            self.rmods_proj_inactive.add_entry(projectee_item_key, raw_modifier);
                        }
                        _ => (),
                    }
                }
            }
            AffecteeFilter::OwnSrq(srq_a_item_id) => {
                if let UadItem::Ship(projectee_ship) = projectee_item {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                    remove_ctx_modifier(
                        &mut self.cmods_own_srq,
                        &(projectee_ship.get_fit_key(), srq_a_item_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    self.rmods_proj_inactive.add_entry(projectee_item_key, raw_modifier);
                }
            }
        }
    }
}
