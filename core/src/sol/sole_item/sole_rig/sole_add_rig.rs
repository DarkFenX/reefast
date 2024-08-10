use crate::{
    defs::{EItemId, SolFitId},
    sol::{
        err::basic::{FitFoundError, ItemAllocError},
        item::{SolItem, SolRig},
        item_info::SolRigInfo,
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn add_rig(&mut self, fit_id: SolFitId, a_item_id: EItemId, state: bool) -> Result<SolRigInfo, AddRigError> {
        let item_id = self.items.alloc_item_id()?;
        let rig = SolRig::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SolRigInfo::from(&rig);
        let item = SolItem::Rig(rig);
        let fit = self.fits.get_fit_mut(&fit_id)?;
        fit.rigs.insert(item_id);
        self.items.add_item(item);
        self.add_item_id_to_svcs(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum AddRigError {
    FitNotFound(FitFoundError),
    ItemIdAllocFailed(ItemAllocError),
}
impl From<FitFoundError> for AddRigError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<ItemAllocError> for AddRigError {
    fn from(error: ItemAllocError) -> Self {
        Self::ItemIdAllocFailed(error)
    }
}
impl std::error::Error for AddRigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::ItemIdAllocFailed(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddRigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::ItemIdAllocFailed(e) => e.fmt(f),
        }
    }
}
