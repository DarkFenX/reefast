use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, FitKey, ItemKey, ItemTypeId, SolarSystem,
        info::FwEffectInfo,
        uad::item::{FwEffect, Item},
    },
};

impl SolarSystem {
    pub fn add_fw_effect(
        &mut self,
        fit_id: &FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<FwEffectInfo, AddFwEffectError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        let item_key = self.add_fw_effect_internal(fit_key, type_id, state);
        Ok(self.get_fw_effect_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn add_fw_effect_internal(
        &mut self,
        fit_key: FitKey,
        type_id: ItemTypeId,
        state: bool,
    ) -> ItemKey {
        let item_id = self.uad.items.alloc_id();
        let fw_effect = FwEffect::new(&self.uad.src, item_id, type_id, fit_key, state);
        let item = Item::FwEffect(fw_effect);
        let item_key = self.uad.items.add(item);
        let fit = self.uad.fits.get_mut(fit_key);
        fit.fw_effects.insert(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
    }
}

#[derive(Debug)]
pub enum AddFwEffectError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for AddFwEffectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddFwEffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddFwEffectError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
