use crate::{
    ac, ad,
    def::ItemKey,
    misc::{AttrSpec, EffectSpec},
    svc::{efuncs, vast::Vast},
    uad::UadItem,
};

impl Vast {
    pub(in crate::svc) fn effect_projected(
        &mut self,
        projector_key: ItemKey,
        projector_item: &UadItem,
        a_effect: &ad::AEffectRt,
        projectee_key: ItemKey,
        projectee_item: &UadItem,
    ) {
        if a_effect.ae.category == ac::effcats::TARGET {
            if !a_effect.ae.stop_ids.is_empty()
                && let Some(projectee_fit_key) = projectee_item.get_fit_key()
            {
                let projectee_fit_data = self.fit_datas.get_mut(&projectee_fit_key).unwrap();
                let stopper = EffectSpec::new(projector_key, a_effect.ae.id);
                for stop_a_effect_id in a_effect.ae.stop_ids.iter() {
                    let stopped = EffectSpec::new(projectee_key, *stop_a_effect_id);
                    projectee_fit_data.stopped_effects.add_entry(stopped, stopper);
                }
            }
            if let Some(projector_fit_key) = projector_item.get_fit_key() {
                let projector_fit_data = self.fit_datas.get_mut(&projector_fit_key).unwrap();
                let projector_espec = EffectSpec::new(projector_key, a_effect.ae.id);
                if a_effect.ae.is_assist {
                    projector_fit_data
                        .blockable_assistance
                        .add_entry(projectee_key, projector_espec);
                }
                if is_offense_blockable(projector_item, a_effect) {
                    projector_fit_data
                        .blockable_offense
                        .add_entry(projectee_key, projector_espec);
                }
                if let Some(resist_a_attr_id) = efuncs::get_resist_a_attr_id(projector_item, a_effect) {
                    let projectee_aspec = AttrSpec::new(projectee_key, resist_a_attr_id);
                    projector_fit_data
                        .resist_immunity
                        .add_entry(projectee_aspec, projector_espec);
                }
            }
        }
        if let Some(rep_getter) = a_effect.hc.get_remote_shield_rep_opc {
            self.irr_shield
                .add_entry(projectee_key, projector_key, a_effect.ae.id, rep_getter);
            if a_effect.hc.charge.is_some() {
                self.irr_shield_limitable
                    .add_entry(projectee_key, projector_key, a_effect.ae.id, rep_getter);
            }
        }
        if let Some(rep_getter) = a_effect.hc.get_remote_armor_rep_opc {
            self.irr_armor
                .add_entry(projectee_key, projector_key, a_effect.ae.id, rep_getter);
            if a_effect.hc.charge.is_some() {
                self.irr_armor_limitable
                    .add_entry(projectee_key, projector_key, a_effect.ae.id, rep_getter);
            }
        }
        if let Some(rep_getter) = a_effect.hc.get_remote_hull_rep_opc {
            self.irr_hull
                .add_entry(projectee_key, projector_key, a_effect.ae.id, rep_getter);
        }
    }
    pub(in crate::svc) fn effect_unprojected(
        &mut self,
        projector_key: ItemKey,
        projector_item: &UadItem,
        a_effect: &ad::AEffectRt,
        projectee_key: ItemKey,
        projectee_item: &UadItem,
    ) {
        if a_effect.ae.category == ac::effcats::TARGET {
            if !a_effect.ae.stop_ids.is_empty()
                && let Some(projectee_fit_key) = projectee_item.get_fit_key()
            {
                let projectee_fit_data = self.fit_datas.get_mut(&projectee_fit_key).unwrap();
                let stopper = EffectSpec::new(projector_key, a_effect.ae.id);
                for stop_a_effect_id in a_effect.ae.stop_ids.iter() {
                    let stopped = EffectSpec::new(projectee_key, *stop_a_effect_id);
                    projectee_fit_data.stopped_effects.remove_entry(&stopped, &stopper);
                }
            }
            if let Some(projector_fit_key) = projector_item.get_fit_key() {
                let projector_fit_data = self.fit_datas.get_mut(&projector_fit_key).unwrap();
                let projector_espec = EffectSpec::new(projector_key, a_effect.ae.id);
                if a_effect.ae.is_assist {
                    projector_fit_data
                        .blockable_assistance
                        .remove_entry(&projectee_key, &projector_espec);
                }
                if is_offense_blockable(projector_item, a_effect) {
                    projector_fit_data
                        .blockable_offense
                        .remove_entry(&projectee_key, &projector_espec);
                }
                if let Some(resist_a_attr_id) = efuncs::get_resist_a_attr_id(projector_item, a_effect) {
                    let projectee_aspec = AttrSpec::new(projectee_key, resist_a_attr_id);
                    projector_fit_data
                        .resist_immunity
                        .remove_entry(&projectee_aspec, &projector_espec);
                }
            }
        }
        if a_effect.hc.get_remote_shield_rep_opc.is_some() {
            self.irr_shield
                .remove_l3(&projectee_key, &projector_key, &a_effect.ae.id);
            if a_effect.hc.charge.is_some() {
                self.irr_shield_limitable
                    .remove_l3(&projectee_key, &projector_key, &a_effect.ae.id);
            }
        }
        if a_effect.hc.get_remote_armor_rep_opc.is_some() {
            self.irr_armor
                .remove_l3(&projectee_key, &projector_key, &a_effect.ae.id);
            if a_effect.hc.charge.is_some() {
                self.irr_armor_limitable
                    .remove_l3(&projectee_key, &projector_key, &a_effect.ae.id);
            }
        }
        if a_effect.hc.get_remote_hull_rep_opc.is_some() {
            self.irr_hull.remove_l3(&projectee_key, &projector_key, &a_effect.ae.id);
        }
    }
}

fn is_offense_blockable(projector_item: &UadItem, a_effect: &ad::AEffectRt) -> bool {
    if a_effect.ae.is_offense && !a_effect.ae.mods.is_empty() {
        return true;
    };
    // Assistance with extra flag can be blocked by the disallow offensive modifiers flag too
    a_effect.ae.is_assist && projector_item.get_a_xt().unwrap().disallow_vs_ew_immune_tgt
}
