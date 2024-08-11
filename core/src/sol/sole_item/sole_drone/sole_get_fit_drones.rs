use crate::{
    defs::SolFitId,
    err::basic::FitFoundError,
    sol::{item_info::SolDroneInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_drones(&self, fit_id: &SolFitId) -> Result<Vec<SolDroneInfo>, GetFitDronesError> {
        let fit = self.fits.get_fit(fit_id)?;
        let drone_infos = fit
            .drones
            .iter()
            .map(|v| SolDroneInfo::from(self.items.get_item(v).unwrap().get_drone().unwrap()))
            .collect();
        Ok(drone_infos)
    }
}

#[derive(Debug)]
pub enum GetFitDronesError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for GetFitDronesError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
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
