use crate::sol::svc::vast::{SolResValFail, SolSlotValFail};

pub struct SolValOptions {
    pub cpu: bool,
    pub powergrid: bool,
    pub calibration: bool,
    pub dronebay_volume: bool,
    pub drone_bandwidth: bool,
    pub rig_slots: bool,
    pub subsystem_slots: bool,
    pub launched_drones: bool,
    pub launched_fighters: bool,
}
impl SolValOptions {
    pub fn new(
        cpu: bool,
        powergrid: bool,
        calibration: bool,
        dronebay_volume: bool,
        drone_bandwidth: bool,
        rig_slots: bool,
        subsystem_slots: bool,
        launched_drones: bool,
        launched_fighters: bool,
    ) -> Self {
        Self {
            cpu,
            powergrid,
            calibration,
            dronebay_volume,
            drone_bandwidth,
            rig_slots,
            subsystem_slots,
            launched_drones,
            launched_fighters,
        }
    }
    pub fn new_enabled() -> Self {
        Self {
            cpu: true,
            powergrid: true,
            calibration: true,
            dronebay_volume: true,
            drone_bandwidth: true,
            rig_slots: true,
            subsystem_slots: true,
            launched_drones: true,
            launched_fighters: true,
        }
    }
    pub fn new_disabled() -> Self {
        Self {
            cpu: false,
            powergrid: false,
            calibration: false,
            dronebay_volume: false,
            drone_bandwidth: false,
            rig_slots: false,
            subsystem_slots: false,
            launched_drones: false,
            launched_fighters: false,
        }
    }
}

pub struct SolValResult {
    pub cpu: Option<SolResValFail>,
    pub powergrid: Option<SolResValFail>,
    pub calibration: Option<SolResValFail>,
    pub dronebay_volume: Option<SolResValFail>,
    pub drone_bandwidth: Option<SolResValFail>,
    pub rig_slots: Option<SolSlotValFail>,
    pub subsystem_slots: Option<SolSlotValFail>,
    pub launched_drones: Option<SolSlotValFail>,
    pub launched_fighters: Option<SolSlotValFail>,
}
impl SolValResult {
    pub(in crate::sol::svc::vast) fn new() -> Self {
        Self {
            cpu: None,
            powergrid: None,
            calibration: None,
            dronebay_volume: None,
            drone_bandwidth: None,
            rig_slots: None,
            subsystem_slots: None,
            launched_drones: None,
            launched_fighters: None,
        }
    }
    pub fn all_passed(&self) -> bool {
        self.cpu.is_none()
            && self.powergrid.is_none()
            && self.calibration.is_none()
            && self.dronebay_volume.is_none()
            && self.drone_bandwidth.is_none()
            && self.rig_slots.is_none()
            && self.subsystem_slots.is_none()
            && self.launched_drones.is_none()
            && self.launched_fighters.is_none()
    }
}
