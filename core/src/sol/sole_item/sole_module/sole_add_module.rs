use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, FitKey, ItemKey, ItemTypeId, ModRack, SolarSystem,
        info::ModuleInfo,
        uad::item::{Charge, Item, ItemAddMutation, Module, ModuleState},
    },
};

use super::{
    pos_modes::{AddMode, RmMode},
    shared::get_fit_rack,
};

impl SolarSystem {
    pub fn add_module(
        &mut self,
        fit_id: &FitId,
        rack: ModRack,
        pos_mode: AddMode,
        type_id: ItemTypeId,
        state: ModuleState,
        mutation: Option<ItemAddMutation>,
        charge_type_id: Option<ItemTypeId>,
    ) -> Result<ModuleInfo, AddModuleError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        let item_key = self.add_module_internal(fit_key, rack, pos_mode, type_id, state, mutation, charge_type_id);
        Ok(self.get_module_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn add_module_internal(
        &mut self,
        fit_key: FitKey,
        rack: ModRack,
        pos_mode: AddMode,
        type_id: ItemTypeId,
        state: ModuleState,
        mutation: Option<ItemAddMutation>,
        charge_type_id: Option<ItemTypeId>,
    ) -> ItemKey {
        let module_item_id = self.uad.items.alloc_id();
        let fit_rack = get_fit_rack(&mut self.uad.fits, fit_key, rack);
        // Assume some random position for now; it will be overwritten later
        let module = Module::new(
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
        let module_item = Item::Module(module);
        let module_key = self.uad.items.add(module_item);
        // Calculate position for the module and update part of user data (fit rack and modules from
        // it)
        let pos = match pos_mode {
            // Add to the end of module rack
            AddMode::Append => fit_rack.append(module_key),
            // Take first spare slot in the rack
            AddMode::Equip => fit_rack.equip(module_key),
            // Insert at specified position, shifting other modules to the right
            AddMode::Insert(pos) => {
                // True means inserted module is not the last in the rack
                if fit_rack.insert(pos, module_key) {
                    for (i, rack_module_key) in fit_rack.inner()[pos + 1..].iter().enumerate() {
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
                match fit_rack.get(pos) {
                    Some(old_module_key) => {
                        self.remove_module_internal(old_module_key, RmMode::Free).unwrap();
                        let fit_rack = get_fit_rack(&mut self.uad.fits, fit_key, rack);
                        fit_rack.place(pos, module_key);
                    }
                    None => fit_rack.place(pos, module_key),
                }
                pos
            }
        };
        // Create and add charge
        let charge_key = match charge_type_id {
            Some(charge_type_id) => {
                let charge_item_id = self.uad.items.alloc_id();
                // Update user data with new charge info
                let charge = Charge::new(
                    &self.uad.src,
                    charge_item_id,
                    charge_type_id,
                    fit_key,
                    module_key,
                    state.into(),
                    false,
                );
                let item = Item::Charge(charge);
                let charge_key = self.uad.items.add(item);
                Some(charge_key)
            }
            None => None,
        };
        // Update on-module data regarding position and charge
        let module = self.uad.items.get_mut(module_key).get_module_mut().unwrap();
        module.set_pos(pos);
        module.set_charge_item_key(charge_key);
        // Add module and charge to services
        self.add_item_key_to_svc(module_key);
        if let Some(charge_key) = charge_key {
            self.add_item_key_to_svc(charge_key);
        }
        module_key
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddModuleError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
