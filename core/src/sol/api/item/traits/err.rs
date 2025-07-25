use crate::{
    err::basic::{ItemLoadedError, SupportedStatError},
    svc::err::StatItemCheckError,
    ud::UItems,
};

#[derive(thiserror::Error, Debug)]
pub enum GetItemAttrError {
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
}

#[derive(thiserror::Error, Debug)]
pub enum IterItemAttrsError {
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
}

#[derive(thiserror::Error, Debug)]
pub enum IterItemEffectsError {
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
}

#[derive(thiserror::Error, Debug)]
pub enum IterItemModifiersError {
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
}

#[derive(thiserror::Error, Debug)]
pub enum ItemStatError {
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error("{0}")]
    UnsupportedStat(#[from] SupportedStatError),
}
impl ItemStatError {
    pub(crate) fn from_svc_err(u_items: &UItems, svc_err: StatItemCheckError) -> Self {
        match svc_err {
            StatItemCheckError::ItemNotLoaded(svc_err) => ItemLoadedError::from_svc_err(u_items, svc_err).into(),
            StatItemCheckError::UnsupportedStat(svc_err) => SupportedStatError::from_svc_err(u_items, svc_err).into(),
        }
    }
}
