use crate::{
    defs::{EAttrId, SolItemId},
    sol::{
        err::basic::{ItemFoundError, ItemLoadedError},
        svc::err::LoadedItemFoundError,
        SolAttrVal, SolView, SolarSystem,
    },
};

impl SolarSystem {
    pub fn iter_item_attrs(
        &mut self,
        item_id: &SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EAttrId, SolAttrVal)>, IterItemAttrsError> {
        let attrs = self
            .svcs
            .calc_iter_item_attr_vals(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item_id)?;
        Ok(attrs)
    }
}

#[derive(Debug)]
pub enum IterItemAttrsError {
    ItemNotFound(ItemFoundError),
    ItemNotLoaded(ItemLoadedError),
}
impl From<LoadedItemFoundError> for IterItemAttrsError {
    fn from(error: LoadedItemFoundError) -> Self {
        match error {
            LoadedItemFoundError::ItemNotFound(e) => Self::ItemNotFound(e),
            LoadedItemFoundError::ItemNotLoaded(e) => Self::ItemNotLoaded(e),
        }
    }
}
impl std::error::Error for IterItemAttrsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemNotLoaded(e) => Some(e),
        }
    }
}
impl std::fmt::Display for IterItemAttrsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemNotLoaded(e) => e.fmt(f),
        }
    }
}
