use crate::{misc::SecZone, sol::SolarSystem};

impl SolarSystem {
    pub fn set_sec_zone(&mut self, sec_zone: SecZone) {
        self.u_data.sec_zone = sec_zone;
        self.svc.notify_sol_sec_zone_changed(&self.u_data);
    }
}
