use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::DroneInfo},
};

impl SolarSystem {
    pub fn get_fit_drones(&self, fit_id: &FitId) -> Result<Vec<DroneInfo>, GetFitDronesError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_drones_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_drones_internal(&self, fit_key: FitKey) -> Vec<DroneInfo> {
        self.uad
            .fits
            .get(fit_key)
            .drones
            .iter()
            .map(|item_key| self.get_drone_internal(*item_key).unwrap())
            .collect()
    }
}

#[derive(Debug)]
pub enum GetFitDronesError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitDronesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitDronesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitDronesError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
