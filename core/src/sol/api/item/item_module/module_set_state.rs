use crate::{
    def::ItemKey,
    misc::ModuleState,
    sol::{SolarSystem, api::ModuleMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_module_state(&mut self, item_key: ItemKey, state: ModuleState) {
        // Update user data for module
        let uad_module = self.uad.items.get_mut(item_key).get_module_mut().unwrap();
        let charge_key = uad_module.get_charge_key();
        let old_a_state = uad_module.get_a_state();
        uad_module.set_module_state(state);
        let new_a_state = uad_module.get_a_state();
        // Update services for module
        SolarSystem::util_switch_item_state(
            &self.uad,
            &mut self.svc,
            &mut self.reffs,
            item_key,
            old_a_state,
            new_a_state,
        );
        if let Some(charge_key) = charge_key {
            // Update user data for charge
            let uad_charge = self.uad.items.get_mut(charge_key).get_charge_mut().unwrap();
            let old_a_state = uad_charge.get_a_state();
            uad_charge.set_a_state(state.into());
            let new_a_state = uad_charge.get_a_state();
            // Update services for charge
            SolarSystem::util_switch_item_state(
                &self.uad,
                &mut self.svc,
                &mut self.reffs,
                charge_key,
                old_a_state,
                new_a_state,
            );
        }
    }
}

impl<'a> ModuleMut<'a> {
    pub fn set_state(&mut self, state: ModuleState) {
        self.sol.internal_set_module_state(self.key, state)
    }
}
