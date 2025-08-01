use rc::Lender;

use crate::{
    info::{HItemInfo, HItemInfoMode, MkItemInfo},
    shared::{HDpsProfile, HFitSecStatus},
};

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFitInfoFull {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::FitId,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet: Option<rc::FleetId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    character: Option<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    skills: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    implants: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    boosters: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ship: Option<HItemInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stance: Option<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    subsystems: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "HModuleRacks::is_empty")]
    modules: HModuleRacks,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    rigs: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    services: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    drones: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fighters: Vec<HItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fw_effects: Vec<HItemInfo>,
    sec_status: HFitSecStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    rah_incoming_dps: Option<HDpsProfile>,
}
impl HFitInfoFull {
    pub(in crate::info::fit) fn mk_info(core_fit: &mut rc::FitMut, item_mode: HItemInfoMode) -> Self {
        Self {
            id: core_fit.get_fit_id(),
            fleet: core_fit.get_fleet().map(|v| v.get_fleet_id()),
            character: core_fit
                .get_character_mut()
                .map(|mut core_character| HItemInfo::mk_info(&mut core_character, item_mode)),
            skills: core_fit
                .iter_skills_mut()
                .map_into_iter(|mut core_skill| HItemInfo::mk_info(&mut core_skill, item_mode))
                .collect(),
            implants: core_fit
                .iter_implants_mut()
                .map_into_iter(|mut core_implant| HItemInfo::mk_info(&mut core_implant, item_mode))
                .collect(),
            boosters: core_fit
                .iter_boosters_mut()
                .map_into_iter(|mut core_booster| HItemInfo::mk_info(&mut core_booster, item_mode))
                .collect(),
            ship: core_fit
                .get_ship_mut()
                .map(|mut core_ship| HItemInfo::mk_info(&mut core_ship, item_mode)),
            stance: core_fit
                .get_stance_mut()
                .map(|mut core_stance| HItemInfo::mk_info(&mut core_stance, item_mode)),
            subsystems: core_fit
                .iter_subsystems_mut()
                .map_into_iter(|mut core_subsystem| HItemInfo::mk_info(&mut core_subsystem, item_mode))
                .collect(),
            modules: HModuleRacks {
                high: core_fit
                    .iter_modules_mut(rc::ModRack::High)
                    .map_into_iter(|core_module| {
                        core_module.map(|mut core_module| HItemInfo::mk_info(&mut core_module, item_mode))
                    })
                    .collect(),
                mid: core_fit
                    .iter_modules_mut(rc::ModRack::Mid)
                    .map_into_iter(|core_module| {
                        core_module.map(|mut core_module| HItemInfo::mk_info(&mut core_module, item_mode))
                    })
                    .collect(),
                low: core_fit
                    .iter_modules_mut(rc::ModRack::Low)
                    .map_into_iter(|core_module| {
                        core_module.map(|mut core_module| HItemInfo::mk_info(&mut core_module, item_mode))
                    })
                    .collect(),
            },
            rigs: core_fit
                .iter_rigs_mut()
                .map_into_iter(|mut core_rig| HItemInfo::mk_info(&mut core_rig, item_mode))
                .collect(),
            services: core_fit
                .iter_services_mut()
                .map_into_iter(|mut core_service| HItemInfo::mk_info(&mut core_service, item_mode))
                .collect(),
            drones: core_fit
                .iter_drones_mut()
                .map_into_iter(|mut core_drone| HItemInfo::mk_info(&mut core_drone, item_mode))
                .collect(),
            fighters: core_fit
                .iter_fighters_mut()
                .map_into_iter(|mut core_fighter| HItemInfo::mk_info(&mut core_fighter, item_mode))
                .collect(),
            fw_effects: core_fit
                .iter_fw_effects_mut()
                .map_into_iter(|mut core_fw_effect| HItemInfo::mk_info(&mut core_fw_effect, item_mode))
                .collect(),
            sec_status: core_fit.get_sec_status().get_inner().into_inner(),
            rah_incoming_dps: core_fit.get_rah_incoming_dps().map(|v| v.into()),
        }
    }
}

#[derive(serde::Serialize)]
struct HModuleRacks {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    high: Vec<Option<HItemInfo>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    mid: Vec<Option<HItemInfo>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    low: Vec<Option<HItemInfo>>,
}
impl HModuleRacks {
    fn is_empty(&self) -> bool {
        self.high.is_empty() && self.mid.is_empty() && self.low.is_empty()
    }
}
