use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::FitInfo},
};

impl SolarSystem {
    pub fn get_fit(&self, fit_id: &FitId) -> Result<FitInfo, GetFitError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_internal(&self, fit_key: FitKey) -> FitInfo {
        let fit = self.uad.fits.get(fit_key);
        FitInfo::from_fit(&self.uad, fit)
    }
}

#[derive(Debug)]
pub enum GetFitError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
