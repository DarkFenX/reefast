use crate::{
    def::FitKey,
    err::basic::FitDpsProfileFoundError,
    sol::{SolarSystem, api::FitMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_fit_rah_incoming_dps(
        &mut self,
        fit_key: FitKey,
    ) -> Result<(), FitDpsProfileFoundError> {
        let fit = self.uad.fits.get_mut(fit_key);
        let old_dps_profile = fit.rah_incoming_dps.take();
        match old_dps_profile {
            Some(old_dps_profile) => {
                // Do not trigger anything in services if effectively RAH profile is not changed -
                // RAH sim uses default incoming dps if RAH profile is not set
                if self.uad.default_incoming_dps != old_dps_profile {
                    self.svc.notify_fit_rah_dps_profile_changed(&self.uad, &fit_key);
                }
            }
            None => return Err(FitDpsProfileFoundError { fit_id: fit.id }),
        }
        Ok(())
    }
}

impl<'a> FitMut<'a> {
    pub fn remove_rah_incoming_dps(&mut self) -> Result<(), RemoveFitRahIncomingDpsError> {
        self.sol.internal_remove_fit_rah_incoming_dps(self.key)?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveFitRahIncomingDpsError {
    #[error("{0}")]
    DpsProfileNotSet(#[from] FitDpsProfileFoundError),
}
