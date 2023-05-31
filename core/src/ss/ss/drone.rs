use crate::{
    consts::State,
    defs::{ReeId, ReeInt},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_drone_info(&self, item_id: &ReeId) -> Result<ssn::DroneInfo> {
        Ok(self.get_drone(item_id)?.into())
    }
    pub fn get_fit_drone_infos(&self, fit_id: &ReeId) -> Vec<ssn::DroneInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                ssi::Item::Drone(d) if d.fit_id == *fit_id => Some(d.into()),
                _ => None,
            })
            .collect()
    }
    pub fn add_drone(&mut self, fit_id: ReeId, type_id: ReeInt, state: State) -> Result<ssn::DroneInfo> {
        let item_id = self.alloc_item_id()?;
        let drone = ssi::Drone::new(&self.src, item_id, fit_id, type_id, state);
        let info = ssn::DroneInfo::from(&drone);
        let item = ssi::Item::Drone(drone);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_drone_state(&mut self, item_id: &ReeId, state: State) -> Result<()> {
        self.get_drone_mut(item_id)?.state = state;
        Ok(())
    }
    // Non-public
    fn get_drone(&self, item_id: &ReeId) -> Result<&ssi::Drone> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::Item::Drone(drone) => Ok(drone),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::Drone::get_name(),
            ))),
        }
    }
    fn get_drone_mut(&mut self, item_id: &ReeId) -> Result<&mut ssi::Drone> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::Item::Drone(drone) => Ok(drone),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::Drone::get_name(),
            ))),
        }
    }
}
