use crate::{
    err::basic::FitFoundError,
    sol::{FitId, SolarSystem, info::ShipInfo},
};

impl SolarSystem {
    pub fn get_fit_ship(&self, fit_id: &FitId) -> Result<Option<ShipInfo>, GetFitShipError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        Ok(fit.ship.map(|item_key| self.get_ship_internal(item_key).unwrap()))
    }
}

#[derive(Debug)]
pub enum GetFitShipError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitShipError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitShipError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitShipError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
