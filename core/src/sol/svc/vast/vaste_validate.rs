use crate::sol::{
    svc::{
        calc::SolCalc,
        vast::{SolValOptions, SolValResult, SolVast},
    },
    uad::{fit::SolFit, SolUad},
};

impl SolVast {
    pub(in crate::sol) fn validate_fit_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        options: SolValOptions,
    ) -> bool {
        // All registered fits should have an entry, so just unwrap
        let fit_data = self.get_fit_data(&fit.id).unwrap();
        if options.cpu {
            if !fit_data.validate_cpu_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.powergrid {
            if !fit_data.validate_powergrid_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.calibration {
            if !fit_data.validate_calibration_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.dronebay_volume {
            if !fit_data.validate_dronebay_volume_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.drone_bandwidth {
            if !fit_data.validate_drone_bandwidth_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.rig_slots {
            if !fit_data.validate_rig_slots_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.subsystem_slots {
            if !fit_data.validate_subsystem_slots_fast(uad, calc, fit) {
                return false;
            }
        }
        if options.launched_drones {
            if !fit_data.validate_launched_drones_fast(uad, calc, fit) {
                return false;
            }
        }
        true
    }
    pub(in crate::sol) fn validate_fit_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        options: SolValOptions,
    ) -> SolValResult {
        // All registered fits should have an entry, so just unwrap
        let fit_data = self.get_fit_data(&fit.id).unwrap();
        let mut result = SolValResult::new();
        if options.cpu {
            result.cpu = fit_data.validate_cpu_verbose(uad, calc, fit);
        }
        if options.powergrid {
            result.powergrid = fit_data.validate_powergrid_verbose(uad, calc, fit);
        }
        if options.calibration {
            result.calibration = fit_data.validate_calibration_verbose(uad, calc, fit);
        }
        if options.dronebay_volume {
            result.dronebay_volume = fit_data.validate_dronebay_volume_verbose(uad, calc, fit);
        }
        if options.drone_bandwidth {
            result.drone_bandwidth = fit_data.validate_drone_bandwidth_verbose(uad, calc, fit);
        }
        if options.rig_slots {
            result.rig_slots = fit_data.validate_rig_slots_verbose(uad, calc, fit);
        }
        if options.subsystem_slots {
            result.subsystem_slots = fit_data.validate_subsystem_slots_verbose(uad, calc, fit);
        }
        if options.launched_drones {
            result.launched_drones = fit_data.validate_launched_drones_verbose(uad, calc, fit);
        }
        result
    }
}
