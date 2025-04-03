use crate::{
    ac, ad,
    sol::{svc::vast::Vast, uad::item::Item},
};

impl Vast {
    pub(in crate::sol::svc) fn effects_started(&mut self, item: &Item, a_effects: &[ad::ArcEffect]) {
        match item {
            Item::Module(module) => {
                for a_effect in a_effects {
                    match a_effect.id {
                        ac::effects::TURRET_FITTED => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_turret.insert(module.get_item_id());
                        }
                        ac::effects::LAUNCHER_FITTED => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_launcher.insert(module.get_item_id());
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::sol::svc) fn effects_stopped(&mut self, item: &Item, a_effects: &[ad::ArcEffect]) {
        match item {
            Item::Module(module) => {
                for a_effect in a_effects {
                    match a_effect.id {
                        ac::effects::TURRET_FITTED => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_turret.remove(&module.get_item_id());
                        }
                        ac::effects::LAUNCHER_FITTED => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_launcher.remove(&module.get_item_id());
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
}
