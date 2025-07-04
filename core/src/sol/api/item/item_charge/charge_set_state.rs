use crate::{
    def::ItemKey,
    sol::{SolarSystem, api::ChargeMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_charge_state(&mut self, item_key: ItemKey, state: bool) {
        let uad_charge = self.uad.items.get_mut(item_key).get_charge_mut().unwrap();
        let old_a_state = uad_charge.get_a_state();
        uad_charge.set_force_disable(!state);
        let new_a_state = uad_charge.get_a_state();
        SolarSystem::util_switch_item_state(
            &self.uad,
            &mut self.svc,
            &mut self.reffs,
            item_key,
            old_a_state,
            new_a_state,
        );
    }
}

impl<'a> ChargeMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_charge_state(self.key, state)
    }
}
