use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::RigInfo},
};

impl SolarSystem {
    pub fn get_rig(&self, item_id: &ItemId) -> Result<RigInfo, GetRigError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_rig_internal(item_key)?)
    }
    pub(in crate::sol) fn get_rig_internal(&self, item_key: ItemKey) -> Result<RigInfo, ItemKindMatchError> {
        let rig = self.uad.items.get(item_key).get_rig()?;
        Ok(RigInfo::from_rig(&self.uad, rig))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetRigError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotRig(#[from] ItemKindMatchError),
}
