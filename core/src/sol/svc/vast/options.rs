use crate::{sol::ItemId, util::RSet};

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
}
impl ValOptions {
    pub fn all_enabled() -> Self {
        Self {
            cpu: ValOption::enabled(),
            powergrid: ValOption::enabled(),
            calibration: ValOption::enabled(),
            drone_bay_volume: ValOption::enabled(),
            drone_bandwidth: ValOption::enabled(),
            fighter_bay_volume: ValOption::enabled(),
            rig_slot_count: ValOption::enabled(),
            service_slot_count: ValOption::enabled(),
            subsystem_slot_count: ValOption::enabled(),
            launched_drone_count: ValOption::enabled(),
            launched_fighter_count: ValOption::enabled(),
            launched_support_fighter_count: ValOption::enabled(),
            launched_light_fighter_count: ValOption::enabled(),
            launched_heavy_fighter_count: ValOption::enabled(),
            launched_standup_support_fighter_count: ValOption::enabled(),
            launched_standup_light_fighter_count: ValOption::enabled(),
            launched_standup_heavy_fighter_count: ValOption::enabled(),
            turret_slot_count: ValOption::enabled(),
            launcher_slot_count: ValOption::enabled(),
            high_slot_count: ValOption::enabled(),
            mid_slot_count: ValOption::enabled(),
            low_slot_count: ValOption::enabled(),
            implant_slot_index: ValOption::enabled(),
            booster_slot_index: ValOption::enabled(),
            subsystem_slot_index: ValOption::enabled(),
            ship_limit: ValOption::enabled(),
            max_group_fitted: ValOption::enabled(),
            max_group_online: ValOption::enabled(),
            max_group_active: ValOption::enabled(),
            rig_size: ValOption::enabled(),
            skill_reqs: ValOption::enabled(),
            charge_group: ValOption::enabled(),
            charge_size: ValOption::enabled(),
            charge_volume: ValOption::enabled(),
            capital_module: ValOption::enabled(),
            not_loaded_item: ValOption::enabled(),
            module_state: ValOption::enabled(),
            item_kind: ValOption::enabled(),
            drone_group: ValOption::enabled(),
            fighter_squad_size: ValOption::enabled(),
            unlaunchable_drone_slot: ValOption::enabled(),
            unlaunchable_drone_bandwidth: ValOption::enabled(),
            unlaunchable_fighter: ValOption::enabled(),
            unlaunchable_support_fighter: ValOption::enabled(),
            unlaunchable_light_fighter: ValOption::enabled(),
            unlaunchable_heavy_fighter: ValOption::enabled(),
            unlaunchable_standup_support_fighter: ValOption::enabled(),
            unlaunchable_standup_light_fighter: ValOption::enabled(),
            unlaunchable_standup_heavy_fighter: ValOption::enabled(),
            ship_stance: ValOption::enabled(),
            overload_skill: ValOption::enabled(),
            max_type_fitted: ValOption::enabled(),
            sec_zone_fitted: ValOption::enabled(),
            sec_zone_online: ValOption::enabled(),
            sec_zone_active: ValOption::enabled(),
            sec_zone_unonlineable: ValOption::enabled(),
            sec_zone_unactivable: ValOption::enabled(),
            activation_blocked: ValOption::enabled(),
            item_vs_ship_kind: ValOption::enabled(),
        }
    }
    pub fn all_disabled() -> Self {
        Self {
            cpu: ValOption::disabled(),
            powergrid: ValOption::disabled(),
            calibration: ValOption::disabled(),
            drone_bay_volume: ValOption::disabled(),
            drone_bandwidth: ValOption::disabled(),
            fighter_bay_volume: ValOption::disabled(),
            rig_slot_count: ValOption::disabled(),
            service_slot_count: ValOption::disabled(),
            subsystem_slot_count: ValOption::disabled(),
            launched_drone_count: ValOption::disabled(),
            launched_fighter_count: ValOption::disabled(),
            launched_support_fighter_count: ValOption::disabled(),
            launched_light_fighter_count: ValOption::disabled(),
            launched_heavy_fighter_count: ValOption::disabled(),
            launched_standup_support_fighter_count: ValOption::disabled(),
            launched_standup_light_fighter_count: ValOption::disabled(),
            launched_standup_heavy_fighter_count: ValOption::disabled(),
            turret_slot_count: ValOption::disabled(),
            launcher_slot_count: ValOption::disabled(),
            high_slot_count: ValOption::disabled(),
            mid_slot_count: ValOption::disabled(),
            low_slot_count: ValOption::disabled(),
            implant_slot_index: ValOption::disabled(),
            booster_slot_index: ValOption::disabled(),
            subsystem_slot_index: ValOption::disabled(),
            ship_limit: ValOption::disabled(),
            max_group_fitted: ValOption::disabled(),
            max_group_online: ValOption::disabled(),
            max_group_active: ValOption::disabled(),
            rig_size: ValOption::disabled(),
            skill_reqs: ValOption::disabled(),
            charge_group: ValOption::disabled(),
            charge_size: ValOption::disabled(),
            charge_volume: ValOption::disabled(),
            capital_module: ValOption::disabled(),
            not_loaded_item: ValOption::disabled(),
            module_state: ValOption::disabled(),
            item_kind: ValOption::disabled(),
            drone_group: ValOption::disabled(),
            fighter_squad_size: ValOption::disabled(),
            unlaunchable_drone_slot: ValOption::disabled(),
            unlaunchable_drone_bandwidth: ValOption::disabled(),
            unlaunchable_fighter: ValOption::disabled(),
            unlaunchable_support_fighter: ValOption::disabled(),
            unlaunchable_light_fighter: ValOption::disabled(),
            unlaunchable_heavy_fighter: ValOption::disabled(),
            unlaunchable_standup_support_fighter: ValOption::disabled(),
            unlaunchable_standup_light_fighter: ValOption::disabled(),
            unlaunchable_standup_heavy_fighter: ValOption::disabled(),
            ship_stance: ValOption::disabled(),
            overload_skill: ValOption::disabled(),
            max_type_fitted: ValOption::disabled(),
            sec_zone_fitted: ValOption::disabled(),
            sec_zone_online: ValOption::disabled(),
            sec_zone_active: ValOption::disabled(),
            sec_zone_unonlineable: ValOption::disabled(),
            sec_zone_unactivable: ValOption::disabled(),
            activation_blocked: ValOption::disabled(),
            item_vs_ship_kind: ValOption::disabled(),
        }
    }
}

#[derive(Clone)]
pub struct ValOption {
    pub enabled: bool,
    pub kfs: RSet<ItemId>,
}
impl ValOption {
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            kfs: RSet::new(),
        }
    }
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            kfs: RSet::new(),
        }
    }
}
