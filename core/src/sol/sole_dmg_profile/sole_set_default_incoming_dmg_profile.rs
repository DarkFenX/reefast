use crate::{
    defs::OF,
    err::basic::{EmDmgNonNegError, ExplDmgNonNegError, KinDmgNonNegError, ThermDmgNonNegError, TotalDmgPositiveError},
    sol::{SolDmgProfile, SolView, SolarSystem},
};

impl SolarSystem {
    pub fn set_default_incoming_dmg_profile(
        &mut self,
        dmg_profile: SolDmgProfile,
    ) -> Result<(), SetDefaultIncomingDmgProfileError> {
        if dmg_profile.em < OF(0.0) {
            return Err(EmDmgNonNegError::new(dmg_profile.em).into());
        }
        if dmg_profile.thermal < OF(0.0) {
            return Err(ThermDmgNonNegError::new(dmg_profile.thermal).into());
        }
        if dmg_profile.kinetic < OF(0.0) {
            return Err(KinDmgNonNegError::new(dmg_profile.kinetic).into());
        }
        if dmg_profile.explosive < OF(0.0) {
            return Err(ExplDmgNonNegError::new(dmg_profile.explosive).into());
        }
        let total = dmg_profile.em + dmg_profile.thermal + dmg_profile.kinetic + dmg_profile.explosive;
        if total <= OF(0.0) {
            return Err(TotalDmgPositiveError::new(total).into());
        }
        if self.default_incoming_dmg != dmg_profile {
            self.default_incoming_dmg = dmg_profile;
            self.svcs.default_incoming_dmg_profile_changed(&SolView::new(
                &self.src,
                &self.fleets,
                &self.fits,
                &self.items,
                &self.default_incoming_dmg,
            ));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetDefaultIncomingDmgProfileError {
    EmDmgNegative(EmDmgNonNegError),
    ThermDmgNegative(ThermDmgNonNegError),
    KinDmgNegative(KinDmgNonNegError),
    ExplDmgNegative(ExplDmgNonNegError),
    TotalDmgNonPositive(TotalDmgPositiveError),
}
impl std::error::Error for SetDefaultIncomingDmgProfileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::EmDmgNegative(e) => Some(e),
            Self::ThermDmgNegative(e) => Some(e),
            Self::KinDmgNegative(e) => Some(e),
            Self::ExplDmgNegative(e) => Some(e),
            Self::TotalDmgNonPositive(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetDefaultIncomingDmgProfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EmDmgNegative(e) => e.fmt(f),
            Self::ThermDmgNegative(e) => e.fmt(f),
            Self::KinDmgNegative(e) => e.fmt(f),
            Self::ExplDmgNegative(e) => e.fmt(f),
            Self::TotalDmgNonPositive(e) => e.fmt(f),
        }
    }
}
impl From<EmDmgNonNegError> for SetDefaultIncomingDmgProfileError {
    fn from(error: EmDmgNonNegError) -> Self {
        Self::EmDmgNegative(error)
    }
}
impl From<ThermDmgNonNegError> for SetDefaultIncomingDmgProfileError {
    fn from(error: ThermDmgNonNegError) -> Self {
        Self::ThermDmgNegative(error)
    }
}
impl From<KinDmgNonNegError> for SetDefaultIncomingDmgProfileError {
    fn from(error: KinDmgNonNegError) -> Self {
        Self::KinDmgNegative(error)
    }
}
impl From<ExplDmgNonNegError> for SetDefaultIncomingDmgProfileError {
    fn from(error: ExplDmgNonNegError) -> Self {
        Self::ExplDmgNegative(error)
    }
}
impl From<TotalDmgPositiveError> for SetDefaultIncomingDmgProfileError {
    fn from(error: TotalDmgPositiveError) -> Self {
        Self::TotalDmgNonPositive(error)
    }
}
