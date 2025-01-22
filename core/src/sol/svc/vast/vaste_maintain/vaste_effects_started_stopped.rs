use crate::{
    ad, ec,
    sol::{svc::vast::SolVast, uad::item::SolItem},
};

impl SolVast {
    pub(in crate::sol::svc) fn effects_started(&mut self, item: &SolItem, effects: &[ad::ArcEffect]) {
        match item {
            SolItem::Module(module) => {
                for effect in effects {
                    match effect.id {
                        ec::effects::ONLINE => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_online.insert(module.get_id());
                        }
                        ec::effects::TURRET_FITTED => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_turret.insert(module.get_id());
                        }
                        ec::effects::LAUNCHER_FITTED => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_launcher.insert(module.get_id());
                        }
                        _ => (),
                    }
                }
            }
            SolItem::Rig(rig) => {
                for effect in effects {
                    if effect.id == ec::effects::RIG_SLOT {
                        if let Some(val) = rig.get_attrs().unwrap().get(&ec::attrs::UPGRADE_COST) {
                            let fit_data = self.get_fit_data_mut(&rig.get_fit_id()).unwrap();
                            fit_data.rigs_rigslot_calibration.insert(rig.get_id(), *val);
                        }
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::sol::svc) fn effects_stopped(&mut self, item: &SolItem, effects: &[ad::ArcEffect]) {
        match item {
            SolItem::Module(module) => {
                for effect in effects {
                    match effect.id {
                        ec::effects::ONLINE => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_online.remove(&module.get_id());
                        }
                        ec::effects::TURRET_FITTED => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_turret.remove(&module.get_id());
                        }
                        ec::effects::LAUNCHER_FITTED => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_launcher.remove(&module.get_id());
                        }
                        _ => (),
                    }
                }
            }
            SolItem::Rig(rig) => {
                for effect in effects {
                    if effect.id == ec::effects::RIG_SLOT {
                        let fit_data = self.get_fit_data_mut(&rig.get_fit_id()).unwrap();
                        fit_data.rigs_rigslot_calibration.remove(&rig.get_id());
                    }
                }
            }
            _ => (),
        }
    }
}
