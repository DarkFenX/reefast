use crate::sol::{
    svc::calc::{
        registers::SolStandardRegister, SolAffecteeFilter, SolCtxModifier, SolLocation, SolLocationKind, SolRawModifier,
    },
    uad::item::{SolItem, SolShipKind},
};

use super::{add_ctx_modifier, remove_ctx_modifier};

impl SolStandardRegister {
    pub(super) fn proj_buff_mod(
        &mut self,
        raw_modifier: SolRawModifier,
        projectee_item: &SolItem,
    ) -> Option<SolCtxModifier> {
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(loc) => match loc {
                SolLocation::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        SolShipKind::Ship => {
                            let ctx_modifier =
                                SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                            add_ctx_modifier(
                                &mut self.cmods_root,
                                (projectee_ship.get_fit_id(), SolLocationKind::Ship),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ if projectee_item.is_buffable() => {
                        let item_id = projectee_item.get_id();
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, item_id);
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
                SolLocation::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        add_ctx_modifier(
                            &mut self.cmods_root,
                            (projectee_ship.get_fit_id(), SolLocationKind::Ship),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::Loc(SolLocation::Everything | SolLocation::Ship) => match projectee_item {
                SolItem::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), SolShipKind::Ship) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                    add_ctx_modifier(
                        &mut self.cmods_loc,
                        (projectee_ship.get_fit_id(), SolLocationKind::Ship),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocGrp(SolLocation::Everything | SolLocation::Ship, grp_id) => match projectee_item {
                SolItem::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), SolShipKind::Ship) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                    add_ctx_modifier(
                        &mut self.cmods_loc_grp,
                        (projectee_ship.get_fit_id(), SolLocationKind::Ship, grp_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocSrq(SolLocation::Everything | SolLocation::Ship, srq_id) => match projectee_item {
                SolItem::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), SolShipKind::Ship) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                    add_ctx_modifier(
                        &mut self.cmods_loc_srq,
                        (projectee_ship.get_fit_id(), SolLocationKind::Ship, srq_id),
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
    pub(super) fn unproj_buff_mod(
        &mut self,
        raw_modifier: SolRawModifier,
        projectee_item: &SolItem,
    ) -> Option<SolCtxModifier> {
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(loc) => match loc {
                SolLocation::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        SolShipKind::Ship => {
                            let ctx_modifier =
                                SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                            remove_ctx_modifier(
                                &mut self.cmods_root,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Ship),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ if projectee_item.is_buffable() => {
                        let item_id = projectee_item.get_id();
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, item_id);
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
                SolLocation::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        remove_ctx_modifier(
                            &mut self.cmods_root,
                            &(projectee_ship.get_fit_id(), SolLocationKind::Ship),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::Loc(SolLocation::Everything | SolLocation::Ship) => match projectee_item {
                SolItem::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), SolShipKind::Ship) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                    remove_ctx_modifier(
                        &mut self.cmods_loc,
                        &(projectee_ship.get_fit_id(), SolLocationKind::Ship),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocGrp(SolLocation::Everything | SolLocation::Ship, grp_id) => match projectee_item {
                SolItem::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), SolShipKind::Ship) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                    remove_ctx_modifier(
                        &mut self.cmods_loc_grp,
                        &(projectee_ship.get_fit_id(), SolLocationKind::Ship, grp_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocSrq(SolLocation::Everything | SolLocation::Ship, srq_id) => match projectee_item {
                SolItem::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), SolShipKind::Ship) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                    remove_ctx_modifier(
                        &mut self.cmods_loc_srq,
                        &(projectee_ship.get_fit_id(), SolLocationKind::Ship, srq_id),
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
