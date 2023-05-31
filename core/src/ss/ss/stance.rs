use itertools::Itertools;

use crate::{
    defs::{ReeId, ReeInt},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_fit_stance_info(&self, fit_id: &ReeId) -> Result<ssn::StanceInfo> {
        self.get_fit_stance(fit_id).map(|v| v.into())
    }
    pub fn set_fit_stance(&mut self, fit_id: ReeId, type_id: ReeInt, state: bool) -> Result<ssn::StanceInfo> {
        match self.remove_fit_stance(&fit_id) {
            Ok(_) => (),
            // Suppress ItemNotFound error, since this method is supposed to be used
            // even when no stance is set
            Err(e) => match e.kind {
                ErrorKind::ItemTypeNotFound(_) => (),
                _ => return Err(e),
            },
        };
        let item_id = self.alloc_item_id()?;
        let stance = ssi::Stance::new(&self.src, item_id, fit_id, type_id, state);
        let info = ssn::StanceInfo::from(&stance);
        let item = ssi::Item::Stance(stance);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_fit_stance_state(&mut self, fit_id: &ReeId, state: bool) -> Result<()> {
        self.get_fit_stance_mut(fit_id)?.set_bool_state(state);
        Ok(())
    }
    pub fn remove_fit_stance(&mut self, fit_id: &ReeId) -> Result<()> {
        self.check_fit(fit_id)?;
        let removed = self
            .items
            .drain_filter(|_, v| match v {
                ssi::Item::Stance(s) if s.fit_id == *fit_id => true,
                _ => false,
            })
            .collect_vec();
        match removed.is_empty() {
            true => Err(Error::new(ErrorKind::ItemTypeNotFound(ssi::Stance::get_name()))),
            false => Ok(()),
        }
    }
    // Non-public
    fn get_fit_stance(&self, fit_id: &ReeId) -> Result<&ssi::Stance> {
        self.items
            .values()
            .find_map(|v| match v {
                ssi::Item::Stance(s) if s.fit_id == *fit_id => Some(s),
                _ => None,
            })
            .ok_or_else(|| Error::new(ErrorKind::ItemTypeNotFound(ssi::Stance::get_name())))
    }
    fn get_fit_stance_mut(&mut self, fit_id: &ReeId) -> Result<&mut ssi::Stance> {
        self.items
            .values_mut()
            .find_map(|v| match v {
                ssi::Item::Stance(s) if s.fit_id == *fit_id => Some(s),
                _ => None,
            })
            .ok_or_else(|| Error::new(ErrorKind::ItemTypeNotFound(ssi::Stance::get_name())))
    }
}
