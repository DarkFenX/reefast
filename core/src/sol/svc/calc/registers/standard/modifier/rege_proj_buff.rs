use crate::sol::{
    svc::calc::{AffecteeFilter, CtxModifier, Location, LocationKind, RawModifier, registers::StandardRegister},
    uad::item::{Item, ShipKind},
};

use super::{add_ctx_modifier, remove_ctx_modifier};

impl StandardRegister {
    pub(super) fn proj_buff_mod(&mut self, raw_modifier: RawModifier, projectee_item: &Item) -> Option<CtxModifier> {
        match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => match projectee_item {
                    Item::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier =
                                CtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_item_id());
                            add_ctx_modifier(
                                &mut self.cmods_root,
                                (projectee_ship.get_fit_id(), LocationKind::Ship),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ if projectee_item.is_buffable() => {
                        let item_id = projectee_item.get_item_id();
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, item_id);
                        add_ctx_modifier(
                            &mut self.cmods_direct,
                            item_id,
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                Location::Ship => match projectee_item {
                    Item::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), ShipKind::Ship) => {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_item_id());
                        add_ctx_modifier(
                            &mut self.cmods_root,
                            (projectee_ship.get_fit_id(), LocationKind::Ship),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::Loc(Location::Everything | Location::Ship) => match projectee_item {
                Item::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), ShipKind::Ship) => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_item_id());
                    add_ctx_modifier(
                        &mut self.cmods_loc,
                        (projectee_ship.get_fit_id(), LocationKind::Ship),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => match projectee_item {
                Item::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), ShipKind::Ship) => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_item_id());
                    add_ctx_modifier(
                        &mut self.cmods_loc_grp,
                        (projectee_ship.get_fit_id(), LocationKind::Ship, a_item_grp_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => match projectee_item {
                Item::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), ShipKind::Ship) => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_item_id());
                    add_ctx_modifier(
                        &mut self.cmods_loc_srq,
                        (projectee_ship.get_fit_id(), LocationKind::Ship, srq_a_item_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            _ => None,
        }
    }
    pub(super) fn unproj_buff_mod(&mut self, raw_modifier: RawModifier, projectee_item: &Item) -> Option<CtxModifier> {
        match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => match projectee_item {
                    Item::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier =
                                CtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_item_id());
                            remove_ctx_modifier(
                                &mut self.cmods_root,
                                &(projectee_ship.get_fit_id(), LocationKind::Ship),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ if projectee_item.is_buffable() => {
                        let item_id = projectee_item.get_item_id();
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, item_id);
                        remove_ctx_modifier(
                            &mut self.cmods_direct,
                            &item_id,
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                Location::Ship => match projectee_item {
                    Item::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), ShipKind::Ship) => {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_item_id());
                        remove_ctx_modifier(
                            &mut self.cmods_root,
                            &(projectee_ship.get_fit_id(), LocationKind::Ship),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::Loc(Location::Everything | Location::Ship) => match projectee_item {
                Item::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), ShipKind::Ship) => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_item_id());
                    remove_ctx_modifier(
                        &mut self.cmods_loc,
                        &(projectee_ship.get_fit_id(), LocationKind::Ship),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => match projectee_item {
                Item::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), ShipKind::Ship) => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_item_id());
                    remove_ctx_modifier(
                        &mut self.cmods_loc_grp,
                        &(projectee_ship.get_fit_id(), LocationKind::Ship, a_item_grp_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => match projectee_item {
                Item::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), ShipKind::Ship) => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_item_id());
                    remove_ctx_modifier(
                        &mut self.cmods_loc_srq,
                        &(projectee_ship.get_fit_id(), LocationKind::Ship, srq_a_item_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            _ => None,
        }
    }
}
