#[derive(thiserror::Error, Debug)]
pub(crate) enum HExecError {
    // Fits
    #[error("{0}")]
    FitNotFoundPrimary(#[source] rc::err::basic::FitFoundError),
    #[error("{0}")]
    FitNotFoundSecondary(#[source] rc::err::basic::FitFoundError),
    #[error("fit {0} has no character set")]
    FitCharacterNotFound(rc::FitId),
    #[error("fit {0} has no ship set")]
    FitShipNotFound(rc::FitId),
    #[error("fit {0} has no stance set")]
    FitStanceNotFound(rc::FitId),
    #[error("{0}")]
    FitIsNotInFleet(#[source] rc::err::basic::FitFleetAssignedError),
    #[error("fit {1} is not in fleet {0}")]
    FitIsNotInThisFleet(rc::FleetId, rc::FitId),
    #[error("{0}")]
    FitItemKindNotFound(#[source] rc::err::basic::FitHasItemKindError),
    // Fleets
    #[error("{0}")]
    FleetNotFoundPrimary(#[source] rc::err::basic::FleetFoundError),
    #[error("{0}")]
    FleetNotFoundSecondary(#[source] rc::err::basic::FleetFoundError),
    // Items
    #[error("{0}")]
    ItemNotFoundPrimary(#[source] rc::err::basic::ItemFoundError),
    #[error("{0}")]
    ItemNotFoundSecondary(#[source] rc::err::basic::ItemFoundError),
    #[error("{0}")]
    ItemKindMismatch(#[source] rc::err::basic::ItemKindMatchError),
    #[error("{0}")]
    SkillIdCollision(#[source] rc::err::basic::SkillEveTypeError),
    #[error("{0}")]
    MutationNotSet(#[source] rc::err::basic::ItemMutatedError),
    #[error("{0}")]
    ChargeNotSet(#[source] rc::err::basic::ChargeFoundError),
    #[error("{0}")]
    UnremovableAutocharge(#[source] rc::err::basic::ItemKindRemoveError),
    #[error("{0}")]
    InvalidSkillLevel(#[source] rc::err::basic::SkillLevelError),
    #[error("{0}")]
    InvalidFighterCount(#[source] rc::err::basic::FighterCountError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[source] rc::err::basic::ItemReceiveProjError),
    #[error("{0}")]
    ProjectionNotFound(#[source] rc::err::basic::ProjFoundError),
    #[error("{0}")]
    ProjectionAlreadyExists(#[source] rc::err::basic::ProjNotFoundError),
    // Misc
    #[error("{0}")]
    InvalidSecStatus(#[source] rc::err::basic::SecStatusError),
    #[error("{0}")]
    InvalidDpsProfile(#[source] rc::err::basic::DmgError),
    #[error("{0}")]
    InvalidBreacher(#[source] rc::err::basic::BreacherDmgError),
}
impl HExecError {
    pub(crate) fn get_code(&self) -> String {
        match self {
            // Fits
            HExecError::FitNotFoundPrimary(_) => "EXC-002",
            HExecError::FitNotFoundSecondary(_) => "EXC-003",
            HExecError::FitCharacterNotFound(_) => "EXC-004",
            HExecError::FitShipNotFound(_) => "EXC-005",
            HExecError::FitStanceNotFound(_) => "EXC-006",
            HExecError::FitIsNotInFleet(_) => "EXC-007",
            HExecError::FitIsNotInThisFleet(_, _) => "EXC-008",
            HExecError::FitItemKindNotFound(_) => "EXC-009",
            // Fleets
            HExecError::FleetNotFoundPrimary(_) => "EXC-010",
            HExecError::FleetNotFoundSecondary(_) => "EXC-011",
            // Items
            HExecError::ItemNotFoundPrimary(_) => "EXC-013",
            HExecError::ItemNotFoundSecondary(_) => "EXC-014",
            HExecError::ItemKindMismatch(_) => "EXC-015",
            HExecError::SkillIdCollision(_) => "EXC-015.1",
            HExecError::MutationNotSet(_) => "MUT-001",
            HExecError::ChargeNotSet(_) => "NCH-001",
            HExecError::UnremovableAutocharge(_) => "ACH-001",
            HExecError::InvalidSkillLevel(_) => "SKL-018",
            HExecError::InvalidFighterCount(_) => "FTR-019",
            HExecError::ProjecteeCantTakeProjs(_) => "EXC-021",
            HExecError::ProjectionNotFound(_) => "EXC-022",
            HExecError::ProjectionAlreadyExists(_) => "EXC-023",
            // Misc
            HExecError::InvalidSecStatus(_) => "EXC-024-1",
            HExecError::InvalidDpsProfile(_) => "EXC-024",
            HExecError::InvalidBreacher(_) => "EXC-029",
        }
        .to_string()
    }
}
