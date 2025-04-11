#[derive(serde::Deserialize)]
pub(crate) struct HValOptions {
    default: bool,
    #[serde(default)]
    cpu: Option<HValOption>,
    #[serde(default)]
    powergrid: Option<HValOption>,
    #[serde(default)]
    calibration: Option<HValOption>,
    #[serde(default)]
    drone_bay_volume: Option<HValOption>,
    #[serde(default)]
    drone_bandwidth: Option<HValOption>,
    #[serde(default)]
    fighter_bay_volume: Option<HValOption>,
    #[serde(default)]
    rig_slot_count: Option<HValOption>,
    #[serde(default)]
    service_slot_count: Option<HValOption>,
    #[serde(default)]
    subsystem_slot_count: Option<HValOption>,
    #[serde(default)]
    launched_drone_count: Option<HValOption>,
    #[serde(default)]
    launched_fighter_count: Option<HValOption>,
    #[serde(default)]
    launched_support_fighter_count: Option<HValOption>,
    #[serde(default)]
    launched_light_fighter_count: Option<HValOption>,
    #[serde(default)]
    launched_heavy_fighter_count: Option<HValOption>,
    #[serde(default)]
    launched_standup_support_fighter_count: Option<HValOption>,
    #[serde(default)]
    launched_standup_light_fighter_count: Option<HValOption>,
    #[serde(default)]
    launched_standup_heavy_fighter_count: Option<HValOption>,
    #[serde(default)]
    turret_slot_count: Option<HValOption>,
    #[serde(default)]
    launcher_slot_count: Option<HValOption>,
    #[serde(default)]
    high_slot_count: Option<HValOption>,
    #[serde(default)]
    mid_slot_count: Option<HValOption>,
    #[serde(default)]
    low_slot_count: Option<HValOption>,
    #[serde(default)]
    implant_slot_index: Option<HValOption>,
    #[serde(default)]
    booster_slot_index: Option<HValOption>,
    #[serde(default)]
    subsystem_slot_index: Option<HValOption>,
    #[serde(default)]
    ship_limit: Option<HValOption>,
    #[serde(default)]
    max_group_fitted: Option<HValOption>,
    #[serde(default)]
    max_group_online: Option<HValOption>,
    #[serde(default)]
    max_group_active: Option<HValOption>,
    #[serde(default)]
    rig_size: Option<HValOption>,
    #[serde(default)]
    skill_reqs: Option<HValOption>,
    #[serde(default)]
    charge_group: Option<HValOption>,
    #[serde(default)]
    charge_size: Option<HValOption>,
    #[serde(default)]
    charge_volume: Option<HValOption>,
    #[serde(default)]
    capital_module: Option<HValOption>,
    #[serde(default)]
    not_loaded_item: Option<HValOption>,
    #[serde(default)]
    module_state: Option<HValOption>,
    #[serde(default)]
    item_kind: Option<HValOption>,
    #[serde(default)]
    drone_group: Option<HValOption>,
    #[serde(default)]
    fighter_squad_size: Option<HValOption>,
    #[serde(default)]
    unlaunchable_drone_slot: Option<HValOption>,
    #[serde(default)]
    unlaunchable_drone_bandwidth: Option<HValOption>,
    #[serde(default)]
    unlaunchable_fighter: Option<HValOption>,
    #[serde(default)]
    unlaunchable_support_fighter: Option<HValOption>,
    #[serde(default)]
    unlaunchable_light_fighter: Option<HValOption>,
    #[serde(default)]
    unlaunchable_heavy_fighter: Option<HValOption>,
    #[serde(default)]
    unlaunchable_standup_support_fighter: Option<HValOption>,
    #[serde(default)]
    unlaunchable_standup_light_fighter: Option<HValOption>,
    #[serde(default)]
    unlaunchable_standup_heavy_fighter: Option<HValOption>,
    #[serde(default)]
    ship_stance: Option<HValOption>,
    #[serde(default)]
    overload_skill: Option<HValOption>,
    #[serde(default)]
    max_type_fitted: Option<HValOption>,
    #[serde(default)]
    sec_zone_fitted: Option<HValOption>,
    #[serde(default)]
    sec_zone_online: Option<HValOption>,
    #[serde(default)]
    sec_zone_active: Option<HValOption>,
    #[serde(default)]
    sec_zone_unonlineable: Option<HValOption>,
    #[serde(default)]
    sec_zone_unactivable: Option<HValOption>,
    #[serde(default)]
    activation_blocked: Option<HValOption>,
    #[serde(default)]
    item_vs_ship_kind: Option<HValOption>,
}
impl HValOptions {
    pub(crate) fn into_core_val_options(&self, core_sol: &mut rc::SolarSystem) -> rc::val::ValOptions {
        let mut core_options = match self.default {
            true => rc::val::ValOptions::all_enabled(),
            false => rc::val::ValOptions::all_disabled(),
        };
        process_option(core_sol, &self.cpu, &mut core_options.cpu);
        process_option(core_sol, &self.powergrid, &mut core_options.powergrid);
        process_option(core_sol, &self.calibration, &mut core_options.calibration);
        process_option(core_sol, &self.drone_bay_volume, &mut core_options.drone_bay_volume);
        process_option(core_sol, &self.drone_bandwidth, &mut core_options.drone_bandwidth);
        process_option(core_sol, &self.fighter_bay_volume, &mut core_options.fighter_bay_volume);
        process_option(core_sol, &self.rig_slot_count, &mut core_options.rig_slot_count);
        process_option(core_sol, &self.service_slot_count, &mut core_options.service_slot_count);
        process_option(
            core_sol,
            &self.subsystem_slot_count,
            &mut core_options.subsystem_slot_count,
        );
        process_option(
            core_sol,
            &self.launched_drone_count,
            &mut core_options.launched_drone_count,
        );
        process_option(
            core_sol,
            &self.launched_fighter_count,
            &mut core_options.launched_fighter_count,
        );
        process_option(
            core_sol,
            &self.launched_support_fighter_count,
            &mut core_options.launched_support_fighter_count,
        );
        process_option(
            core_sol,
            &self.launched_light_fighter_count,
            &mut core_options.launched_light_fighter_count,
        );
        process_option(
            core_sol,
            &self.launched_heavy_fighter_count,
            &mut core_options.launched_heavy_fighter_count,
        );
        process_option(
            core_sol,
            &self.launched_standup_support_fighter_count,
            &mut core_options.launched_standup_support_fighter_count,
        );
        process_option(
            core_sol,
            &self.launched_standup_light_fighter_count,
            &mut core_options.launched_standup_light_fighter_count,
        );
        process_option(
            core_sol,
            &self.launched_standup_heavy_fighter_count,
            &mut core_options.launched_standup_heavy_fighter_count,
        );
        process_option(core_sol, &self.turret_slot_count, &mut core_options.turret_slot_count);
        process_option(
            core_sol,
            &self.launcher_slot_count,
            &mut core_options.launcher_slot_count,
        );
        process_option(core_sol, &self.high_slot_count, &mut core_options.high_slot_count);
        process_option(core_sol, &self.mid_slot_count, &mut core_options.mid_slot_count);
        process_option(core_sol, &self.low_slot_count, &mut core_options.low_slot_count);
        process_option(core_sol, &self.implant_slot_index, &mut core_options.implant_slot_index);
        process_option(core_sol, &self.booster_slot_index, &mut core_options.booster_slot_index);
        process_option(
            core_sol,
            &self.subsystem_slot_index,
            &mut core_options.subsystem_slot_index,
        );
        process_option(core_sol, &self.ship_limit, &mut core_options.ship_limit);
        process_option(core_sol, &self.max_group_fitted, &mut core_options.max_group_fitted);
        process_option(core_sol, &self.max_group_online, &mut core_options.max_group_online);
        process_option(core_sol, &self.max_group_active, &mut core_options.max_group_active);
        process_option(core_sol, &self.rig_size, &mut core_options.rig_size);
        process_option(core_sol, &self.skill_reqs, &mut core_options.skill_reqs);
        process_option(core_sol, &self.charge_group, &mut core_options.charge_group);
        process_option(core_sol, &self.charge_size, &mut core_options.charge_size);
        process_option(core_sol, &self.charge_volume, &mut core_options.charge_volume);
        process_option(core_sol, &self.capital_module, &mut core_options.capital_module);
        process_option(core_sol, &self.not_loaded_item, &mut core_options.not_loaded_item);
        process_option(core_sol, &self.module_state, &mut core_options.module_state);
        process_option(core_sol, &self.item_kind, &mut core_options.item_kind);
        process_option(core_sol, &self.drone_group, &mut core_options.drone_group);
        process_option(core_sol, &self.fighter_squad_size, &mut core_options.fighter_squad_size);
        process_option(
            core_sol,
            &self.unlaunchable_drone_slot,
            &mut core_options.unlaunchable_drone_slot,
        );
        process_option(
            core_sol,
            &self.unlaunchable_drone_bandwidth,
            &mut core_options.unlaunchable_drone_bandwidth,
        );
        process_option(
            core_sol,
            &self.unlaunchable_fighter,
            &mut core_options.unlaunchable_fighter,
        );
        process_option(
            core_sol,
            &self.unlaunchable_support_fighter,
            &mut core_options.unlaunchable_support_fighter,
        );
        process_option(
            core_sol,
            &self.unlaunchable_light_fighter,
            &mut core_options.unlaunchable_light_fighter,
        );
        process_option(
            core_sol,
            &self.unlaunchable_heavy_fighter,
            &mut core_options.unlaunchable_heavy_fighter,
        );
        process_option(
            core_sol,
            &self.unlaunchable_standup_support_fighter,
            &mut core_options.unlaunchable_standup_support_fighter,
        );
        process_option(
            core_sol,
            &self.unlaunchable_standup_light_fighter,
            &mut core_options.unlaunchable_standup_light_fighter,
        );
        process_option(
            core_sol,
            &self.unlaunchable_standup_heavy_fighter,
            &mut core_options.unlaunchable_standup_heavy_fighter,
        );
        process_option(core_sol, &self.ship_stance, &mut core_options.ship_stance);
        process_option(core_sol, &self.overload_skill, &mut core_options.overload_skill);
        process_option(core_sol, &self.max_type_fitted, &mut core_options.max_type_fitted);
        process_option(core_sol, &self.sec_zone_fitted, &mut core_options.sec_zone_fitted);
        process_option(core_sol, &self.sec_zone_online, &mut core_options.sec_zone_online);
        process_option(core_sol, &self.sec_zone_active, &mut core_options.sec_zone_active);
        process_option(
            core_sol,
            &self.sec_zone_unonlineable,
            &mut core_options.sec_zone_unonlineable,
        );
        process_option(
            core_sol,
            &self.sec_zone_unactivable,
            &mut core_options.sec_zone_unactivable,
        );
        process_option(core_sol, &self.activation_blocked, &mut core_options.activation_blocked);
        process_option(core_sol, &self.item_vs_ship_kind, &mut core_options.item_vs_ship_kind);
        core_options
    }
}
impl Default for HValOptions {
    fn default() -> Self {
        Self {
            default: true,
            cpu: None,
            powergrid: None,
            calibration: None,
            drone_bay_volume: None,
            drone_bandwidth: None,
            fighter_bay_volume: None,
            rig_slot_count: None,
            service_slot_count: None,
            subsystem_slot_count: None,
            launched_drone_count: None,
            launched_fighter_count: None,
            launched_support_fighter_count: None,
            launched_light_fighter_count: None,
            launched_heavy_fighter_count: None,
            launched_standup_support_fighter_count: None,
            launched_standup_light_fighter_count: None,
            launched_standup_heavy_fighter_count: None,
            turret_slot_count: None,
            launcher_slot_count: None,
            high_slot_count: None,
            mid_slot_count: None,
            low_slot_count: None,
            implant_slot_index: None,
            booster_slot_index: None,
            subsystem_slot_index: None,
            ship_limit: None,
            max_group_fitted: None,
            max_group_online: None,
            max_group_active: None,
            rig_size: None,
            skill_reqs: None,
            charge_group: None,
            charge_size: None,
            charge_volume: None,
            capital_module: None,
            not_loaded_item: None,
            module_state: None,
            item_kind: None,
            drone_group: None,
            fighter_squad_size: None,
            unlaunchable_drone_slot: None,
            unlaunchable_drone_bandwidth: None,
            unlaunchable_fighter: None,
            unlaunchable_support_fighter: None,
            unlaunchable_light_fighter: None,
            unlaunchable_heavy_fighter: None,
            unlaunchable_standup_support_fighter: None,
            unlaunchable_standup_light_fighter: None,
            unlaunchable_standup_heavy_fighter: None,
            ship_stance: None,
            overload_skill: None,
            max_type_fitted: None,
            sec_zone_fitted: None,
            sec_zone_online: None,
            sec_zone_active: None,
            sec_zone_unonlineable: None,
            sec_zone_unactivable: None,
            activation_blocked: None,
            item_vs_ship_kind: None,
        }
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
#[serde(untagged)]
enum HValOption {
    Simple(bool),
    Extended(
        bool,
        #[serde_as(as = "Vec<serde_with::DisplayFromStr>")] Vec<rc::ItemId>,
    ),
}
impl HValOption {
    fn is_enabled(&self) -> bool {
        match self {
            Self::Simple(enabled) => *enabled,
            Self::Extended(enabled, _) => *enabled,
        }
    }
    fn get_known_failures(&self) -> Vec<rc::ItemId> {
        match self {
            Self::Simple(_) => Vec::new(),
            Self::Extended(_, known_failures) => known_failures.clone(),
        }
    }
}

fn process_option(core_sol: &rc::SolarSystem, option: &Option<HValOption>, core_option: &mut rc::val::ValOption) {
    if let Some(option) = option {
        core_option.enabled = option.is_enabled();
        core_option.add_known_failures(core_sol, option.get_known_failures().iter())
    }
}
