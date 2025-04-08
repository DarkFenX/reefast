use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, ItemKey, ItemTypeId, SolarSystem,
        info::RigInfo,
        uad::item::{Item, Rig},
    },
};

impl SolarSystem {
    pub fn add_rig(&mut self, fit_id: FitId, type_id: ItemTypeId, state: bool) -> Result<RigInfo, AddRigError> {
        let item_key = self.add_rig_internal(fit_id, type_id, state)?;
        Ok(self.get_rig_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn add_rig_internal(
        &mut self,
        fit_id: FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<ItemKey, FitFoundError> {
        let item_id = self.uad.items.alloc_item_id();
        let rig = Rig::new(&self.uad.src, item_id, type_id, fit_id, state);
        let item = Item::Rig(rig);
        let item_key = self.uad.items.add(item);
        let fit = self.uad.fits.get_fit_mut(&fit_id)?;
        fit.rigs.insert(item_key);
        self.add_item_key_to_svc(item_key);
        Ok(item_key)
    }
}

#[derive(Debug)]
pub enum AddRigError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for AddRigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddRigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddRigError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
