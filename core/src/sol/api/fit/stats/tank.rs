use crate::{
    def::AttrVal,
    misc::{DmgKinds, DpsProfile},
    sol::api::FitMut,
    svc::vast::{StatLayerEhp, StatLayerHp, StatTank},
};

impl<'a> FitMut<'a> {
    pub fn get_stat_hp(&mut self) -> Option<StatTank<StatLayerHp>> {
        self.get_ship_mut().and_then(|mut v| v.get_stat_hp())
    }
    pub fn get_stat_ehp(&mut self, incoming_dps: Option<&DpsProfile>) -> Option<StatTank<StatLayerEhp>> {
        self.get_ship_mut().and_then(|mut v| v.get_stat_ehp(incoming_dps))
    }
    pub fn get_stat_wc_ehp(&mut self) -> Option<StatTank<StatLayerEhp>> {
        self.get_ship_mut().and_then(|mut v| v.get_stat_wc_ehp())
    }
    pub fn get_stat_resists(&mut self) -> Option<StatTank<DmgKinds<AttrVal>>> {
        self.get_ship_mut().and_then(|mut v| v.get_stat_resists())
    }
}
