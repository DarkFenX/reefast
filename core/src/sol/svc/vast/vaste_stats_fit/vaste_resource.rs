use crate::{
    defs::{AttrVal, EAttrId, SolItemId},
    ec,
    sol::{
        svc::{calc::SolCalc, vast::SolVastFitData},
        uad::{fit::SolFit, SolUad},
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
    pub(in crate::sol::svc::vast) fn get_stats_cpu(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatRes {
        self.get_resource_stats(
            uad,
            calc,
            fit,
            self.mods_online.iter(),
            &ec::attrs::CPU,
            &ec::attrs::CPU_OUTPUT,
            true,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_pg(&self, uad: &SolUad, calc: &mut SolCalc, fit: &SolFit) -> SolStatRes {
        self.get_resource_stats(
            uad,
            calc,
            fit,
            self.mods_online.iter(),
            &ec::attrs::POWER,
            &ec::attrs::POWER_OUTPUT,
            true,
        )
    }
    pub(in crate::sol::svc::vast) fn get_stats_calibration(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatRes {
        self.get_resource_stats(
            uad,
            calc,
            fit,
            self.rigs_rigslot.iter(),
            &ec::attrs::UPGRADE_COST,
            &ec::attrs::UPGRADE_CAPACITY,
            false,
        )
    }
    fn get_resource_stats<'a>(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        items: impl Iterator<Item = &'a SolItemId>,
        use_attr_id: &EAttrId,
        output_attr_id: &EAttrId,
        round_used: bool,
    ) -> SolStatRes {
        let output = match fit.ship {
            Some(ship_id) => match calc.get_item_attr_val(uad, &ship_id, output_attr_id) {
                Ok(attr_val) => Some(attr_val.extra),
                Err(_) => None,
            },
            None => None,
        };
        let used = items
            .filter_map(|i| calc.get_item_attr_val(uad, i, use_attr_id).ok().map(|v| v.extra))
            .sum();
        match round_used {
            // Round possible float errors despite individual use values being rounded
            true => SolStatRes::new(round(used, 2), output),
            false => SolStatRes::new(used, output),
        }
    }
}
