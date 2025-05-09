use crate::sol::{
    AddMode, FitKey, ItemKey, ItemMutationRequest, ItemTypeId, ModRack, ModuleState, RmMode, SolarSystem,
    api::{FitMut, ModuleMut},
    uad::item::{UadCharge, UadItem, UadModule},
};

use super::shared::get_fit_rack_mut;

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_module(
        &mut self,
        fit_key: FitKey,
        rack: ModRack,
        pos_mode: AddMode,
        type_id: ItemTypeId,
        state: ModuleState,
        mutation: Option<ItemMutationRequest>,
        charge_type_id: Option<ItemTypeId>,
    ) -> ItemKey {
        let module_item_id = self.uad.items.alloc_id();
        let uad_fit_rack = get_fit_rack_mut(&mut self.uad.fits, fit_key, rack);
        // Assume some random position for now; it will be overwritten later
        let uad_module = UadModule::new(
            &self.uad.src,
            module_item_id,
            type_id,
            fit_key,
            state,
            rack,
            0,
            mutation,
            None,
        );
        let module_uad_item = UadItem::Module(uad_module);
        let module_key = self.uad.items.add(module_uad_item);
        // Calculate position for the module and update part of user data (fit rack and modules from
        // it)
        let pos = match pos_mode {
            // Add to the end of module rack
            AddMode::Append => uad_fit_rack.append(module_key),
            // Take first spare slot in the rack
            AddMode::Equip => uad_fit_rack.equip(module_key),
            // Insert at specified position, shifting other modules to the right
            AddMode::Insert(pos) => {
                // True means inserted module is not the last in the rack
                if uad_fit_rack.insert(pos, module_key) {
                    for (i, rack_module_key) in uad_fit_rack.inner()[pos + 1..].iter().enumerate() {
                        if let Some(rack_module_key) = rack_module_key {
                            self.uad
                                .items
                                .get_mut(*rack_module_key)
                                .get_module_mut()
                                .unwrap()
                                .set_pos(pos + 1 + i);
                        }
                    }
                }
                pos
            }
            // Check if there is a module on position we want to have module, and if yes, remove it
            // before adding new one
            AddMode::Replace(pos) => {
                match uad_fit_rack.get(pos) {
                    Some(old_module_key) => {
                        self.internal_remove_module(old_module_key, RmMode::Free);
                        let uad_fit_rack = get_fit_rack_mut(&mut self.uad.fits, fit_key, rack);
                        uad_fit_rack.place(pos, module_key);
                    }
                    None => uad_fit_rack.place(pos, module_key),
                }
                pos
            }
        };
        // Create and add charge
        let charge_key = match charge_type_id {
            Some(charge_type_id) => {
                let charge_item_id = self.uad.items.alloc_id();
                // Update user data with new charge info
                let uad_charge = UadCharge::new(
                    &self.uad.src,
                    charge_item_id,
                    charge_type_id,
                    fit_key,
                    module_key,
                    state.into(),
                    false,
                );
                let charge_uad_item = UadItem::Charge(uad_charge);
                let charge_key = self.uad.items.add(charge_uad_item);
                Some(charge_key)
            }
            None => None,
        };
        // Update on-module data regarding position and charge
        let uad_module = self.uad.items.get_mut(module_key).get_module_mut().unwrap();
        uad_module.set_pos(pos);
        uad_module.set_charge_item_key(charge_key);
        // Add module and charge to services
        self.internal_add_item_key_to_svc(module_key);
        if let Some(charge_key) = charge_key {
            self.internal_add_item_key_to_svc(charge_key);
        }
        module_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_module(
        &mut self,
        rack: ModRack,
        pos_mode: AddMode,
        type_id: ItemTypeId,
        state: ModuleState,
    ) -> ModuleMut {
        let item_key = self
            .sol
            .internal_add_module(self.key, rack, pos_mode, type_id, state, None, None);
        ModuleMut::new(self.sol, item_key)
    }
}
