use crate::{
    err::basic::FitFoundError,
    sol::{DpsProfile, FitId, FitKey, SolarSystem},
};

impl SolarSystem {
    pub fn set_fit_rah_incoming_dps(
        &mut self,
        fit_id: &FitId,
        dps_profile: DpsProfile,
    ) -> Result<(), SetFitRahIncomingDpsError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        self.set_fit_rah_incoming_dps_internal(fit_key, dps_profile);
        Ok(())
    }
    pub(in crate::sol) fn set_fit_rah_incoming_dps_internal(&mut self, fit_key: FitKey, dps_profile: DpsProfile) {
        let fit = self.uad.fits.get_mut(fit_key);
        if fit.rah_incoming_dps == Some(dps_profile) {
            return;
        }
        let old_dps_profile = fit.rah_incoming_dps.replace(dps_profile);
        // Do not trigger anything in services if effectively RAH profile is not changed - RAH sim
        // uses default incoming dps if RAH profile is not set
        if old_dps_profile.is_none() && self.uad.default_incoming_dps == dps_profile {
            return;
        }
        self.svc.fit_rah_dps_profile_changed(&self.uad, &fit_key);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetFitRahIncomingDpsError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
