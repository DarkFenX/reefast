use crate::{
    ad,
    sol::{
        AttrVal, FitId, ItemKey,
        svc::Svc,
        uad::{
            Uad,
            fleet::Fleet,
            item::{Fighter, Item, Skill},
        },
    },
    src::Src,
};

impl Svc {
    pub(in crate::sol::svc) fn notify_src_changed(&mut self, src: &Src) {
        self.calc.src_changed(src);
    }
    pub(in crate::sol::svc) fn notify_fit_added(&mut self, fit_id: FitId) {
        self.calc.fit_added(fit_id);
        self.vast.fit_added(fit_id);
    }
    pub(in crate::sol::svc) fn notify_fit_removed(&mut self, fit_id: FitId) {
        self.calc.fit_removed(fit_id);
        self.vast.fit_removed(&fit_id);
    }
    pub(in crate::sol::svc) fn notify_fit_added_to_fleet(&mut self, uad: &Uad, fleet: &Fleet, fit_id: &FitId) {
        self.calc.fit_added_to_fleet(uad, fleet, fit_id);
    }
    pub(in crate::sol::svc) fn notify_fit_removed_from_fleet(&mut self, uad: &Uad, fleet: &Fleet, fit_id: &FitId) {
        self.calc.fit_removed_from_fleet(uad, fleet, fit_id);
    }
    pub(in crate::sol::svc) fn notify_fit_rah_dps_profile_changed(&mut self, uad: &Uad, fit_id: &FitId) {
        self.calc.fit_rah_dps_profile_changed(uad, fit_id);
    }
    pub(in crate::sol::svc) fn notify_item_added(&mut self, uad: &Uad, item_key: ItemKey, item: &Item) {
        self.calc.item_added(uad, item_key, item);
        self.vast.item_added(item_key, item);
    }
    pub(in crate::sol::svc) fn notify_item_removed(&mut self, uad: &Uad, item_key: ItemKey, item: &Item) {
        self.calc.item_removed(uad, item_key, item);
        self.vast.item_removed(uad, item_key, item);
    }
    pub(in crate::sol::svc) fn notify_state_activated(&mut self, item_key: ItemKey, item: &Item, a_state: &ad::AState) {
        self.vast.item_state_activated(item_key, item, a_state);
    }
    pub(in crate::sol::svc) fn notify_state_deactivated(
        &mut self,
        item_key: &ItemKey,
        item: &Item,
        a_state: &ad::AState,
    ) {
        self.vast.item_state_deactivated(item_key, item, a_state);
    }
    pub(in crate::sol::svc) fn notify_item_loaded(&mut self, uad: &Uad, item_key: ItemKey, item: &Item) {
        self.calc.item_loaded(uad, item_key, item);
        self.vast.item_loaded(uad, item_key, item);
    }
    pub(in crate::sol::svc) fn notify_item_unloaded(&mut self, uad: &Uad, item_key: ItemKey, item: &Item) {
        self.calc.item_unloaded(uad, item_key, item);
        self.vast.item_unloaded(&item_key, item);
    }
    pub(in crate::sol::svc) fn notify_base_attr_value_changed(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
        a_attr_id: ad::AAttrId,
    ) {
        self.calc.force_attr_value_recalc(uad, item_key, a_attr_id);
    }
    pub(in crate::sol::svc) fn notify_item_state_activated_loaded(
        &mut self,
        item_key: ItemKey,
        item: &Item,
        a_state: &ad::AState,
    ) {
        self.vast.item_state_activated_loaded(item_key, item, a_state);
    }
    pub(in crate::sol::svc) fn notify_item_state_deactivated_loaded(
        &mut self,
        item_key: &ItemKey,
        item: &Item,
        a_state: &ad::AState,
    ) {
        self.vast.item_state_deactivated_loaded(item_key, item, a_state);
    }
    pub(in crate::sol::svc) fn notify_effects_started(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
        item: &Item,
        a_effects: &[ad::ArcEffect],
    ) {
        self.running_effects
            .effects_started(item_key, a_effects.iter().map(|v| v.id));
        self.calc.effects_started(uad, item_key, item, a_effects);
    }
    pub(in crate::sol::svc) fn notify_effects_stopped(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
        item: &Item,
        a_effects: &[ad::ArcEffect],
    ) {
        self.calc.effects_stopped(uad, item_key, item, a_effects);
        self.running_effects
            .effects_stopped(&item_key, a_effects.iter().map(|v| &v.id));
    }
    pub(in crate::sol::svc) fn notify_item_projected(&mut self) {}
    pub(in crate::sol::svc) fn notify_item_unprojected(&mut self) {}
    pub(in crate::sol::svc) fn notify_item_proj_range_changed(&mut self) {}
    pub(in crate::sol::svc) fn notify_effect_projected(
        &mut self,
        uad: &Uad,
        projector_item_key: ItemKey,
        a_effect: &ad::ArcEffect,
        projectee_item_key: ItemKey,
        projectee_item: &Item,
        range: Option<AttrVal>,
    ) {
        self.calc.effect_projected(
            uad,
            projector_item_key,
            a_effect,
            projectee_item_key,
            projectee_item,
            range,
        );
    }
    pub(in crate::sol::svc) fn notify_effect_unprojected(
        &mut self,
        uad: &Uad,
        projector_item_key: ItemKey,
        a_effect: &ad::ArcEffect,
        projectee_item_key: ItemKey,
        projectee_item: &Item,
    ) {
        self.calc
            .effect_unprojected(uad, projector_item_key, a_effect, projectee_item_key, projectee_item);
    }
    pub(in crate::sol::svc) fn notify_effect_proj_range_changed(
        &mut self,
        uad: &Uad,
        projector_item_key: ItemKey,
        effect: &ad::ArcEffect,
        projectee_item_key: ItemKey,
        projectee_item: &Item,
        range: Option<AttrVal>,
    ) {
        self.calc.effect_proj_range_changed(
            uad,
            projector_item_key,
            effect,
            projectee_item_key,
            projectee_item,
            range,
        );
    }
    pub(in crate::sol::svc) fn notify_sol_sec_zone_changed(&mut self, uad: &Uad) {
        self.calc.sol_sec_zone_changed(uad);
    }
    pub(in crate::sol::svc) fn notify_fighter_count_changed(
        &mut self,
        uad: &Uad,
        fighter_key: ItemKey,
        fighter: &Fighter,
    ) {
        self.calc.fighter_count_changed(uad, fighter_key);
        self.vast.fighter_count_changed(fighter_key, fighter);
    }
    pub(in crate::sol::svc) fn notify_ship_sec_status_changed(&mut self, uad: &Uad, ship_key: ItemKey) {
        self.calc.ship_sec_status_changed(uad, ship_key);
    }
    pub(in crate::sol::svc) fn notify_skill_level_changed(&mut self, uad: &Uad, skill_key: ItemKey, skill: &Skill) {
        self.calc.skill_level_changed(uad, skill_key);
        self.vast.skill_level_changed(uad, skill);
    }
}
