use crate::sol::{
    svc::calc::{
        SolAffecteeFilter, SolCtxModifier, SolLocation, SolLocationKind, SolRawModifier, registers::SolStandardRegister,
    },
    uad::item::{SolItem, SolShipKind},
};

use super::{add_ctx_modifier, remove_ctx_modifier};

impl SolStandardRegister {
    pub(super) fn proj_target_mod(
        &mut self,
        raw_modifier: SolRawModifier,
        projectee_item: &SolItem,
    ) -> Option<SolCtxModifier> {
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(loc) => match loc {
                SolLocation::Target => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        match projectee_ship.get_kind() {
                            SolShipKind::Ship => add_ctx_modifier(
                                &mut self.cmods_root,
                                (projectee_ship.get_fit_id(), SolLocationKind::Ship),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            ),
                            SolShipKind::Structure => add_ctx_modifier(
                                &mut self.cmods_root,
                                (projectee_ship.get_fit_id(), SolLocationKind::Structure),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            ),
                            _ => add_ctx_modifier(
                                &mut self.cmods_direct,
                                projectee_ship.get_id(),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            ),
                        }
                        Some(ctx_modifier)
                    }
                    _ => {
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
                },
                _ => None,
            },
            SolAffecteeFilter::Loc(loc) => match loc {
                SolLocation::Target => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        SolShipKind::Ship => {
                            let ctx_modifier =
                                SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                            add_ctx_modifier(
                                &mut self.cmods_loc,
                                (projectee_ship.get_fit_id(), SolLocationKind::Ship),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        SolShipKind::Structure => {
                            let ctx_modifier =
                                SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                            add_ctx_modifier(
                                &mut self.cmods_loc,
                                (projectee_ship.get_fit_id(), SolLocationKind::Structure),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocGrp(loc, grp_id) => match loc {
                SolLocation::Target => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        SolShipKind::Ship => {
                            let ctx_modifier =
                                SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                            add_ctx_modifier(
                                &mut self.cmods_loc_grp,
                                (projectee_ship.get_fit_id(), SolLocationKind::Ship, grp_id),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        SolShipKind::Structure => {
                            let ctx_modifier =
                                SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                            add_ctx_modifier(
                                &mut self.cmods_loc_grp,
                                (projectee_ship.get_fit_id(), SolLocationKind::Structure, grp_id),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocSrq(loc, srq_id) => match loc {
                SolLocation::Target => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        SolShipKind::Ship => {
                            let ctx_modifier =
                                SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                            add_ctx_modifier(
                                &mut self.cmods_loc_srq,
                                (projectee_ship.get_fit_id(), SolLocationKind::Ship, srq_id),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        SolShipKind::Structure => {
                            let ctx_modifier =
                                SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                            add_ctx_modifier(
                                &mut self.cmods_loc_srq,
                                (projectee_ship.get_fit_id(), SolLocationKind::Structure, srq_id),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match projectee_item {
                SolItem::Ship(projectee_ship) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                    add_ctx_modifier(
                        &mut self.cmods_own_srq,
                        (projectee_ship.get_fit_id(), srq_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
        }
    }
    pub(super) fn unproj_target_mod(
        &mut self,
        raw_modifier: SolRawModifier,
        projectee_item: &SolItem,
    ) -> Option<SolCtxModifier> {
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(loc) => match loc {
                SolLocation::Target => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        match projectee_ship.get_kind() {
                            SolShipKind::Ship => remove_ctx_modifier(
                                &mut self.cmods_root,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Ship),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            ),
                            SolShipKind::Structure => remove_ctx_modifier(
                                &mut self.cmods_root,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Structure),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            ),
                            _ => remove_ctx_modifier(
                                &mut self.cmods_direct,
                                &projectee_ship.get_id(),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            ),
                        };
                        Some(ctx_modifier)
                    }
                    _ => {
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
                },
                _ => None,
            },
            SolAffecteeFilter::Loc(loc) => match loc {
                SolLocation::Target => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        SolShipKind::Ship => {
                            let ctx_modifier =
                                SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                            remove_ctx_modifier(
                                &mut self.cmods_loc,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Ship),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        SolShipKind::Structure => {
                            let ctx_modifier =
                                SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                            remove_ctx_modifier(
                                &mut self.cmods_loc,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Structure),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocGrp(loc, grp_id) => match loc {
                SolLocation::Target => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        SolShipKind::Ship => {
                            let ctx_modifier =
                                SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                            remove_ctx_modifier(
                                &mut self.cmods_loc_grp,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Ship, grp_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        SolShipKind::Structure => {
                            let ctx_modifier =
                                SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                            remove_ctx_modifier(
                                &mut self.cmods_loc_grp,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Structure, grp_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocSrq(loc, srq_id) => match loc {
                SolLocation::Target => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        SolShipKind::Ship => {
                            let ctx_modifier =
                                SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                            remove_ctx_modifier(
                                &mut self.cmods_loc_srq,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Ship, srq_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        SolShipKind::Structure => {
                            let ctx_modifier =
                                SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                            remove_ctx_modifier(
                                &mut self.cmods_loc_srq,
                                &(projectee_ship.get_fit_id(), SolLocationKind::Structure, srq_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match projectee_item {
                SolItem::Ship(projectee_ship) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                    remove_ctx_modifier(
                        &mut self.cmods_own_srq,
                        &(projectee_ship.get_fit_id(), srq_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
        }
    }
}
