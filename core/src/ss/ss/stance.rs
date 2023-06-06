use itertools::Itertools;

use crate::{
    defs::{ReeId, ReeInt},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_fit_stance_info(&self, fit_id: &ReeId) -> Result<ssn::SsStanceInfo> {
        self.get_fit_stance(fit_id).map(|v| v.into())
    }
    pub fn set_fit_stance(&mut self, fit_id: ReeId, a_item_id: ReeInt, state: bool) -> Result<ssn::SsStanceInfo> {
        match self.remove_fit_stance(&fit_id) {
            Ok(_) => (),
            // Suppress ItemNotFound error, since this method is supposed to be used
            // even when no stance is set
            Err(e) => match e.kind {
                ErrorKind::SsItemTypeNotFound(_) => (),
                _ => return Err(e),
            },
        };
        let item_id = self.alloc_item_id()?;
        let stance = ssi::SsStance::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = ssn::SsStanceInfo::from(&stance);
        let item = ssi::SsItem::Stance(stance);
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
                ssi::SsItem::Stance(s) if s.fit_id == *fit_id => true,
                _ => false,
            })
            .collect_vec();
        match removed.is_empty() {
            true => Err(Error::new(ErrorKind::SsItemTypeNotFound(ssi::SsStance::get_name()))),
            false => Ok(()),
        }
    }
    // Non-public
    fn get_fit_stance(&self, fit_id: &ReeId) -> Result<&ssi::SsStance> {
        self.items
            .values()
            .find_map(|v| match v {
                ssi::SsItem::Stance(s) if s.fit_id == *fit_id => Some(s),
                _ => None,
            })
            .ok_or_else(|| Error::new(ErrorKind::SsItemTypeNotFound(ssi::SsStance::get_name())))
    }
    fn get_fit_stance_mut(&mut self, fit_id: &ReeId) -> Result<&mut ssi::SsStance> {
        self.items
            .values_mut()
            .find_map(|v| match v {
                ssi::SsItem::Stance(s) if s.fit_id == *fit_id => Some(s),
                _ => None,
            })
            .ok_or_else(|| Error::new(ErrorKind::SsItemTypeNotFound(ssi::SsStance::get_name())))
    }
}
