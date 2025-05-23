use crate::{
    sol::{ItemId, ItemKey, SolarSystem},
    util::RSet,
};

#[derive(Clone)]
pub struct ValOptions {
    pub cpu: ValOption,
    pub powergrid: ValOption,
    pub calibration: ValOption,
    pub drone_bay_volume: ValOption,
    pub drone_bandwidth: ValOption,
    pub fighter_bay_volume: ValOption,
    pub rig_slot_count: ValOption,
    pub service_slot_count: ValOption,
    pub subsystem_slot_count: ValOption,
    pub launched_drone_count: ValOption,
    pub launched_fighter_count: ValOption,
    pub launched_support_fighter_count: ValOption,
    pub launched_light_fighter_count: ValOption,
    pub launched_heavy_fighter_count: ValOption,
    pub launched_standup_support_fighter_count: ValOption,
    pub launched_standup_light_fighter_count: ValOption,
    pub launched_standup_heavy_fighter_count: ValOption,
    pub turret_slot_count: ValOption,
    pub launcher_slot_count: ValOption,
    pub high_slot_count: ValOption,
    pub mid_slot_count: ValOption,
    pub low_slot_count: ValOption,
    pub implant_slot_index: ValOption,
    pub booster_slot_index: ValOption,
    pub subsystem_slot_index: ValOption,
    pub ship_limit: ValOption,
    pub max_group_fitted: ValOption,
    pub max_group_online: ValOption,
    pub max_group_active: ValOption,
    pub rig_size: ValOption,
    pub skill_reqs: ValOption,
    pub charge_group: ValOption,
    pub charge_size: ValOption,
    pub charge_volume: ValOption,
    pub capital_module: ValOption,
    pub not_loaded_item: ValOption,
    pub module_state: ValOption,
    pub item_kind: ValOption,
    pub drone_group: ValOption,
    pub fighter_squad_size: ValOption,
    pub unlaunchable_drone_slot: ValOption,
    pub unlaunchable_drone_bandwidth: ValOption,
    pub unlaunchable_fighter: ValOption,
    pub unlaunchable_support_fighter: ValOption,
    pub unlaunchable_light_fighter: ValOption,
    pub unlaunchable_heavy_fighter: ValOption,
    pub unlaunchable_standup_support_fighter: ValOption,
    pub unlaunchable_standup_light_fighter: ValOption,
    pub unlaunchable_standup_heavy_fighter: ValOption,
    pub ship_stance: ValOption,
    pub overload_skill: ValOption,
    pub max_type_fitted: ValOption,
    pub sec_zone_fitted: ValOption,
    pub sec_zone_online: ValOption,
    pub sec_zone_active: ValOption,
    pub sec_zone_unonlineable: ValOption,
    pub sec_zone_unactivable: ValOption,
    pub activation_blocked: ValOption,
    pub item_vs_ship_kind: ValOption,
    pub effect_stopper: ValOption,
    pub assist_immunity: ValOption,
    pub offense_immunity: ValOption,
}
impl ValOptions {
    pub fn all_enabled() -> Self {
        Self {
            cpu: ValOption::new_enabled(),
            powergrid: ValOption::new_enabled(),
            calibration: ValOption::new_enabled(),
            drone_bay_volume: ValOption::new_enabled(),
            drone_bandwidth: ValOption::new_enabled(),
            fighter_bay_volume: ValOption::new_enabled(),
            rig_slot_count: ValOption::new_enabled(),
            service_slot_count: ValOption::new_enabled(),
            subsystem_slot_count: ValOption::new_enabled(),
            launched_drone_count: ValOption::new_enabled(),
            launched_fighter_count: ValOption::new_enabled(),
            launched_support_fighter_count: ValOption::new_enabled(),
            launched_light_fighter_count: ValOption::new_enabled(),
            launched_heavy_fighter_count: ValOption::new_enabled(),
            launched_standup_support_fighter_count: ValOption::new_enabled(),
            launched_standup_light_fighter_count: ValOption::new_enabled(),
            launched_standup_heavy_fighter_count: ValOption::new_enabled(),
            turret_slot_count: ValOption::new_enabled(),
            launcher_slot_count: ValOption::new_enabled(),
            high_slot_count: ValOption::new_enabled(),
            mid_slot_count: ValOption::new_enabled(),
            low_slot_count: ValOption::new_enabled(),
            implant_slot_index: ValOption::new_enabled(),
            booster_slot_index: ValOption::new_enabled(),
            subsystem_slot_index: ValOption::new_enabled(),
            ship_limit: ValOption::new_enabled(),
            max_group_fitted: ValOption::new_enabled(),
            max_group_online: ValOption::new_enabled(),
            max_group_active: ValOption::new_enabled(),
            rig_size: ValOption::new_enabled(),
            skill_reqs: ValOption::new_enabled(),
            charge_group: ValOption::new_enabled(),
            charge_size: ValOption::new_enabled(),
            charge_volume: ValOption::new_enabled(),
            capital_module: ValOption::new_enabled(),
            not_loaded_item: ValOption::new_enabled(),
            module_state: ValOption::new_enabled(),
            item_kind: ValOption::new_enabled(),
            drone_group: ValOption::new_enabled(),
            fighter_squad_size: ValOption::new_enabled(),
            unlaunchable_drone_slot: ValOption::new_enabled(),
            unlaunchable_drone_bandwidth: ValOption::new_enabled(),
            unlaunchable_fighter: ValOption::new_enabled(),
            unlaunchable_support_fighter: ValOption::new_enabled(),
            unlaunchable_light_fighter: ValOption::new_enabled(),
            unlaunchable_heavy_fighter: ValOption::new_enabled(),
            unlaunchable_standup_support_fighter: ValOption::new_enabled(),
            unlaunchable_standup_light_fighter: ValOption::new_enabled(),
            unlaunchable_standup_heavy_fighter: ValOption::new_enabled(),
            ship_stance: ValOption::new_enabled(),
            overload_skill: ValOption::new_enabled(),
            max_type_fitted: ValOption::new_enabled(),
            sec_zone_fitted: ValOption::new_enabled(),
            sec_zone_online: ValOption::new_enabled(),
            sec_zone_active: ValOption::new_enabled(),
            sec_zone_unonlineable: ValOption::new_enabled(),
            sec_zone_unactivable: ValOption::new_enabled(),
            activation_blocked: ValOption::new_enabled(),
            item_vs_ship_kind: ValOption::new_enabled(),
            effect_stopper: ValOption::new_enabled(),
            assist_immunity: ValOption::new_enabled(),
            offense_immunity: ValOption::new_enabled(),
        }
    }
    pub fn all_disabled() -> Self {
        Self {
            cpu: ValOption::new_disabled(),
            powergrid: ValOption::new_disabled(),
            calibration: ValOption::new_disabled(),
            drone_bay_volume: ValOption::new_disabled(),
            drone_bandwidth: ValOption::new_disabled(),
            fighter_bay_volume: ValOption::new_disabled(),
            rig_slot_count: ValOption::new_disabled(),
            service_slot_count: ValOption::new_disabled(),
            subsystem_slot_count: ValOption::new_disabled(),
            launched_drone_count: ValOption::new_disabled(),
            launched_fighter_count: ValOption::new_disabled(),
            launched_support_fighter_count: ValOption::new_disabled(),
            launched_light_fighter_count: ValOption::new_disabled(),
            launched_heavy_fighter_count: ValOption::new_disabled(),
            launched_standup_support_fighter_count: ValOption::new_disabled(),
            launched_standup_light_fighter_count: ValOption::new_disabled(),
            launched_standup_heavy_fighter_count: ValOption::new_disabled(),
            turret_slot_count: ValOption::new_disabled(),
            launcher_slot_count: ValOption::new_disabled(),
            high_slot_count: ValOption::new_disabled(),
            mid_slot_count: ValOption::new_disabled(),
            low_slot_count: ValOption::new_disabled(),
            implant_slot_index: ValOption::new_disabled(),
            booster_slot_index: ValOption::new_disabled(),
            subsystem_slot_index: ValOption::new_disabled(),
            ship_limit: ValOption::new_disabled(),
            max_group_fitted: ValOption::new_disabled(),
            max_group_online: ValOption::new_disabled(),
            max_group_active: ValOption::new_disabled(),
            rig_size: ValOption::new_disabled(),
            skill_reqs: ValOption::new_disabled(),
            charge_group: ValOption::new_disabled(),
            charge_size: ValOption::new_disabled(),
            charge_volume: ValOption::new_disabled(),
            capital_module: ValOption::new_disabled(),
            not_loaded_item: ValOption::new_disabled(),
            module_state: ValOption::new_disabled(),
            item_kind: ValOption::new_disabled(),
            drone_group: ValOption::new_disabled(),
            fighter_squad_size: ValOption::new_disabled(),
            unlaunchable_drone_slot: ValOption::new_disabled(),
            unlaunchable_drone_bandwidth: ValOption::new_disabled(),
            unlaunchable_fighter: ValOption::new_disabled(),
            unlaunchable_support_fighter: ValOption::new_disabled(),
            unlaunchable_light_fighter: ValOption::new_disabled(),
            unlaunchable_heavy_fighter: ValOption::new_disabled(),
            unlaunchable_standup_support_fighter: ValOption::new_disabled(),
            unlaunchable_standup_light_fighter: ValOption::new_disabled(),
            unlaunchable_standup_heavy_fighter: ValOption::new_disabled(),
            ship_stance: ValOption::new_disabled(),
            overload_skill: ValOption::new_disabled(),
            max_type_fitted: ValOption::new_disabled(),
            sec_zone_fitted: ValOption::new_disabled(),
            sec_zone_online: ValOption::new_disabled(),
            sec_zone_active: ValOption::new_disabled(),
            sec_zone_unonlineable: ValOption::new_disabled(),
            sec_zone_unactivable: ValOption::new_disabled(),
            activation_blocked: ValOption::new_disabled(),
            item_vs_ship_kind: ValOption::new_disabled(),
            effect_stopper: ValOption::new_disabled(),
            assist_immunity: ValOption::new_disabled(),
            offense_immunity: ValOption::new_disabled(),
        }
    }
}

#[derive(Clone)]
pub struct ValOption {
    pub enabled: bool,
    pub(in crate::sol::svc::vast) kfs: RSet<ItemKey>,
}
impl ValOption {
    pub fn new_enabled() -> Self {
        Self {
            enabled: true,
            kfs: RSet::new(),
        }
    }
    pub fn new_disabled() -> Self {
        Self {
            enabled: false,
            kfs: RSet::new(),
        }
    }
    pub fn add_known_failures<'a>(&mut self, sol: &SolarSystem, kfs: impl Iterator<Item = &'a ItemId>) {
        self.kfs
            .extend(kfs.filter_map(|item_id| sol.uad.items.key_by_id(item_id)));
    }
}
