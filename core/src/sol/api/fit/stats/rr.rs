use crate::{def::AttrVal, misc::Spool, sol::api::FitMut};

impl<'a> FitMut<'a> {
    pub fn get_stat_rr_shield(&mut self, spool: Option<Spool>) -> AttrVal {
        self.sol.svc.get_stat_fit_orr_shield(&self.sol.uad, self.key, spool)
    }
    pub fn get_stat_rr_armor(&mut self, spool: Option<Spool>) -> AttrVal {
        self.sol.svc.get_stat_fit_orr_armor(&self.sol.uad, self.key, spool)
    }
    pub fn get_stat_rr_hull(&mut self, spool: Option<Spool>) -> AttrVal {
        self.sol.svc.get_stat_fit_orr_hull(&self.sol.uad, self.key, spool)
    }
    pub fn get_stat_rr_capacitor(&mut self, spool: Option<Spool>) -> AttrVal {
        self.sol.svc.get_stat_fit_orr_cap(&self.sol.uad, self.key, spool)
    }
}
