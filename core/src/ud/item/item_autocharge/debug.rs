use crate::{
    dbg::{DebugError, DebugResult, check_effect_key, check_fit_key, check_item_key},
    ud::{UAutocharge, UData},
};

impl UAutocharge {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        check_fit_key(u_data, self.get_fit_key())?;
        // All autocharges are supposed to be loaded
        check_item_key(u_data, self.get_cont_item_key(), true)?;
        // Autocharges should exist only for effects available on current source
        check_effect_key(u_data, self.get_cont_effect_key())?;
        // Effect key should be available on containing item
        if !u_data
            .items
            .get(self.get_cont_item_key())
            .get_effect_datas()
            .unwrap()
            .contains_key(&self.get_cont_effect_key())
        {
            return Err(DebugError {});
        }
        self.get_projs().consistency_check(u_data)?;
        Ok(())
    }
}
