use crate::{defs::SolItemId, util::StSet};

#[derive(Clone)]
pub struct SolValOptions {
    pub cpu: SolValOption,
    pub powergrid: SolValOption,
    pub calibration: SolValOption,
    pub drone_bay_volume: SolValOption,
    pub drone_bandwidth: SolValOption,
    pub fighter_bay_volume: SolValOption,
    pub rig_slot_count: SolValOption,
    pub service_slot_count: SolValOption,
    pub subsystem_slot_count: SolValOption,
    pub launched_drone_count: SolValOption,
    pub launched_fighter_count: SolValOption,
    pub launched_support_fighter_count: SolValOption,
    pub launched_light_fighter_count: SolValOption,
    pub launched_heavy_fighter_count: SolValOption,
    pub launched_standup_support_fighter_count: SolValOption,
    pub launched_standup_light_fighter_count: SolValOption,
    pub launched_standup_heavy_fighter_count: SolValOption,
    pub turret_slot_count: SolValOption,
    pub launcher_slot_count: SolValOption,
    pub high_slot_count: SolValOption,
    pub mid_slot_count: SolValOption,
    pub low_slot_count: SolValOption,
    pub implant_slot_index: SolValOption,
    pub booster_slot_index: SolValOption,
    pub subsystem_slot_index: SolValOption,
    pub ship_limit: SolValOption,
    pub max_group_fitted: SolValOption,
    pub max_group_online: SolValOption,
    pub max_group_active: SolValOption,
    pub rig_size: SolValOption,
    pub skill_reqs: SolValOption,
    pub charge_group: SolValOption,
    pub charge_size: SolValOption,
    pub charge_volume: SolValOption,
    pub capital_module: SolValOption,
    pub not_loaded_item: SolValOption,
    pub module_state: SolValOption,
    pub item_kind: SolValOption,
    pub drone_group: SolValOption,
    pub fighter_count: SolValOption,
    pub unlaunchable_drone_slot: SolValOption,
    pub unlaunchable_drone_bandwidth: SolValOption,
    pub unlaunchable_fighter: SolValOption,
    pub unlaunchable_support_fighter: SolValOption,
    pub unlaunchable_light_fighter: SolValOption,
    pub unlaunchable_heavy_fighter: SolValOption,
    pub unlaunchable_standup_support_fighter: SolValOption,
    pub unlaunchable_standup_light_fighter: SolValOption,
    pub unlaunchable_standup_heavy_fighter: SolValOption,
    pub ship_stance: SolValOption,
    pub overload_skill: SolValOption,
    pub max_type_fitted: SolValOption,
}
impl SolValOptions {
    pub fn all_enabled() -> Self {
        Self {
            cpu: SolValOption::enabled(),
            powergrid: SolValOption::enabled(),
            calibration: SolValOption::enabled(),
            drone_bay_volume: SolValOption::enabled(),
            drone_bandwidth: SolValOption::enabled(),
            fighter_bay_volume: SolValOption::enabled(),
            rig_slot_count: SolValOption::enabled(),
            service_slot_count: SolValOption::enabled(),
            subsystem_slot_count: SolValOption::enabled(),
            launched_drone_count: SolValOption::enabled(),
            launched_fighter_count: SolValOption::enabled(),
            launched_support_fighter_count: SolValOption::enabled(),
            launched_light_fighter_count: SolValOption::enabled(),
            launched_heavy_fighter_count: SolValOption::enabled(),
            launched_standup_support_fighter_count: SolValOption::enabled(),
            launched_standup_light_fighter_count: SolValOption::enabled(),
            launched_standup_heavy_fighter_count: SolValOption::enabled(),
            turret_slot_count: SolValOption::enabled(),
            launcher_slot_count: SolValOption::enabled(),
            high_slot_count: SolValOption::enabled(),
            mid_slot_count: SolValOption::enabled(),
            low_slot_count: SolValOption::enabled(),
            implant_slot_index: SolValOption::enabled(),
            booster_slot_index: SolValOption::enabled(),
            subsystem_slot_index: SolValOption::enabled(),
            ship_limit: SolValOption::enabled(),
            max_group_fitted: SolValOption::enabled(),
            max_group_online: SolValOption::enabled(),
            max_group_active: SolValOption::enabled(),
            rig_size: SolValOption::enabled(),
            skill_reqs: SolValOption::enabled(),
            charge_group: SolValOption::enabled(),
            charge_size: SolValOption::enabled(),
            charge_volume: SolValOption::enabled(),
            capital_module: SolValOption::enabled(),
            not_loaded_item: SolValOption::enabled(),
            module_state: SolValOption::enabled(),
            item_kind: SolValOption::enabled(),
            drone_group: SolValOption::enabled(),
            fighter_count: SolValOption::enabled(),
            unlaunchable_drone_slot: SolValOption::enabled(),
            unlaunchable_drone_bandwidth: SolValOption::enabled(),
            unlaunchable_fighter: SolValOption::enabled(),
            unlaunchable_support_fighter: SolValOption::enabled(),
            unlaunchable_light_fighter: SolValOption::enabled(),
            unlaunchable_heavy_fighter: SolValOption::enabled(),
            unlaunchable_standup_support_fighter: SolValOption::enabled(),
            unlaunchable_standup_light_fighter: SolValOption::enabled(),
            unlaunchable_standup_heavy_fighter: SolValOption::enabled(),
            ship_stance: SolValOption::enabled(),
            overload_skill: SolValOption::enabled(),
            max_type_fitted: SolValOption::enabled(),
        }
    }
    pub fn all_disabled() -> Self {
        Self {
            cpu: SolValOption::disabled(),
            powergrid: SolValOption::disabled(),
            calibration: SolValOption::disabled(),
            drone_bay_volume: SolValOption::disabled(),
            drone_bandwidth: SolValOption::disabled(),
            fighter_bay_volume: SolValOption::disabled(),
            rig_slot_count: SolValOption::disabled(),
            service_slot_count: SolValOption::disabled(),
            subsystem_slot_count: SolValOption::disabled(),
            launched_drone_count: SolValOption::disabled(),
            launched_fighter_count: SolValOption::disabled(),
            launched_support_fighter_count: SolValOption::disabled(),
            launched_light_fighter_count: SolValOption::disabled(),
            launched_heavy_fighter_count: SolValOption::disabled(),
            launched_standup_support_fighter_count: SolValOption::disabled(),
            launched_standup_light_fighter_count: SolValOption::disabled(),
            launched_standup_heavy_fighter_count: SolValOption::disabled(),
            turret_slot_count: SolValOption::disabled(),
            launcher_slot_count: SolValOption::disabled(),
            high_slot_count: SolValOption::disabled(),
            mid_slot_count: SolValOption::disabled(),
            low_slot_count: SolValOption::disabled(),
            implant_slot_index: SolValOption::disabled(),
            booster_slot_index: SolValOption::disabled(),
            subsystem_slot_index: SolValOption::disabled(),
            ship_limit: SolValOption::disabled(),
            max_group_fitted: SolValOption::disabled(),
            max_group_online: SolValOption::disabled(),
            max_group_active: SolValOption::disabled(),
            rig_size: SolValOption::disabled(),
            skill_reqs: SolValOption::disabled(),
            charge_group: SolValOption::disabled(),
            charge_size: SolValOption::disabled(),
            charge_volume: SolValOption::disabled(),
            capital_module: SolValOption::disabled(),
            not_loaded_item: SolValOption::disabled(),
            module_state: SolValOption::disabled(),
            item_kind: SolValOption::disabled(),
            drone_group: SolValOption::disabled(),
            fighter_count: SolValOption::disabled(),
            unlaunchable_drone_slot: SolValOption::disabled(),
            unlaunchable_drone_bandwidth: SolValOption::disabled(),
            unlaunchable_fighter: SolValOption::disabled(),
            unlaunchable_support_fighter: SolValOption::disabled(),
            unlaunchable_light_fighter: SolValOption::disabled(),
            unlaunchable_heavy_fighter: SolValOption::disabled(),
            unlaunchable_standup_support_fighter: SolValOption::disabled(),
            unlaunchable_standup_light_fighter: SolValOption::disabled(),
            unlaunchable_standup_heavy_fighter: SolValOption::disabled(),
            ship_stance: SolValOption::disabled(),
            overload_skill: SolValOption::disabled(),
            max_type_fitted: SolValOption::disabled(),
        }
    }
}

#[derive(Clone)]
pub struct SolValOption {
    pub enabled: bool,
    pub kfs: StSet<SolItemId>,
}
impl SolValOption {
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            kfs: StSet::new(),
        }
    }
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            kfs: StSet::new(),
        }
    }
}
