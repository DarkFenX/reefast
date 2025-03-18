use crate::{
    defs::{AttrVal, EAttrId, SolItemId},
    ec,
    sol::{
        svc::{calc::SolCalc, vast::SolVastFitData},
        uad::{SolUad, fit::SolFit},
    },
    util::round,
};

pub struct SolStatRes {
    pub used: AttrVal,
    pub output: Option<AttrVal>,
}
impl SolStatRes {
    pub(in crate::sol::svc::vast) fn new(used: AttrVal, output: Option<AttrVal>) -> Self {
        SolStatRes { used, output }
    }
}

impl SolVastFitData {
    // Public methods
    pub(in crate::sol::svc::vast) fn get_stats_cpu(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatRes {
        get_resource_stats_fitting(
            uad,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ec::attrs::CPU,
            &ec::attrs::CPU_OUTPUT,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_powergrid(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatRes {
        get_resource_stats_fitting(
            uad,
            calc,
            fit,
            self.mods_svcs_online.iter(),
            &ec::attrs::POWER,
            &ec::attrs::POWER_OUTPUT,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_calibration(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatRes {
        get_resource_stats_other(
            uad,
            calc,
            fit,
            self.rigs_rigslot_calibration.values(),
            &ec::attrs::UPGRADE_CAPACITY,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_drone_bay_volume(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatRes {
        get_resource_stats_other(uad, calc, fit, self.drones_volume.values(), &ec::attrs::DRONE_CAPACITY)
    }
    pub(in crate::sol::svc::vast) fn get_stats_drone_bandwidth(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatRes {
        get_resource_stats_other(
            uad,
            calc,
            fit,
            self.drones_online_bandwidth.values(),
            &ec::attrs::DRONE_BANDWIDTH,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_fighter_bay_volume(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatRes {
        get_resource_stats_other(uad, calc, fit, self.fighters_volume.values(), &ec::attrs::FTR_CAPACITY)
    }
}

fn get_resource_stats_fitting<'a>(
    uad: &SolUad,
    calc: &mut SolCalc,
    fit: &SolFit,
    items: impl Iterator<Item = &'a SolItemId>,
    use_attr_id: &EAttrId,
    output_attr_id: &EAttrId,
) -> SolStatRes {
    let output = calc.get_item_attr_val_simple_opt(uad, &fit.ship, output_attr_id);
    let used = items
        .filter_map(|i| calc.get_item_attr_val_simple(uad, i, use_attr_id))
        .sum();
    // Round possible float errors despite individual use values being rounded
    SolStatRes::new(round(used, 2), output)
}
fn get_resource_stats_other<'a>(
    uad: &SolUad,
    calc: &mut SolCalc,
    fit: &SolFit,
    items_use: impl Iterator<Item = &'a AttrVal>,
    output_attr_id: &EAttrId,
) -> SolStatRes {
    let output = calc.get_item_attr_val_simple_opt(uad, &fit.ship, output_attr_id);
    let used = items_use.sum();
    SolStatRes::new(used, output)
}
