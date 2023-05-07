use itertools::Itertools;

use crate::{
    ss::item::{Item, Ship, ShipInfo},
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    fn get_fit_ship(&self, fit_id: &ReeId) -> Option<&Ship> {
        self.items.values().find_map(|v| match v {
            Item::Ship(s) if s.fit_id == *fit_id => Some(s),
            _ => None,
        })
    }
    pub fn get_fit_ship_info(&self, fit_id: &ReeId) -> Option<ShipInfo> {
        self.get_fit_ship(fit_id).map(|v| v.into())
    }
    pub fn set_fit_ship(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        match self.remove_fit_ship(&fit_id) {
            Ok(_) => (),
            // Suppress ItemNotFound error, since this method is supposed to be used
            // even when no ship is set
            Err(e) => match e.kind {
                ErrorKind::ItemNotFound => (),
                _ => return Err(e),
            },
        };
        let item_id = self.alloc_item_id()?;
        let ship = Item::Ship(Ship::new(&self.src, item_id, fit_id, type_id));
        self.add_item(ship);
        Ok(item_id)
    }
    pub fn remove_fit_ship(&mut self, fit_id: &ReeId) -> Result<()> {
        self.check_fit(fit_id)?;
        let removed = self
            .items
            .drain_filter(|_, v| match v {
                Item::Ship(s) if s.fit_id == *fit_id => true,
                _ => false,
            })
            .collect_vec();
        match removed.is_empty() {
            true => Err(Error::new(ErrorKind::ItemNotFound, "ship not found")),
            false => Ok(()),
        }
    }
}
