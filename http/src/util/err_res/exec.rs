#[derive(Debug)]
pub(crate) enum HExecError {
    // Fits
    FitNotFoundPrimary(rc::err::basic::FitFoundError),
    FitNotFoundSecondary(rc::err::basic::FitFoundError),
    FitCharacterNotFound(rc::FitId),
    FitShipNotFound(rc::FitId),
    FitStanceNotFound(rc::FitId),
    FitIsNotInFleet(rc::err::basic::FitFleetAssignedError),
    FitIsNotInThisFleet(rc::FleetId, rc::FitId),
    FitItemKindNotFound(rc::err::basic::FitHasItemKindError),
    // Fleets
    FleetNotFoundPrimary(rc::err::basic::FleetFoundError),
    FleetNotFoundSecondary(rc::err::basic::FleetFoundError),
    // Items
    ItemNotFoundPrimary(rc::err::basic::ItemFoundError),
    ItemNotFoundSecondary(rc::err::basic::ItemFoundError),
    ItemKindMismatch(rc::err::basic::ItemKindMatchError),
    SkillIdCollision(rc::err::basic::SkillEveTypeError),
    MutationNotSet(rc::err::basic::ItemMutatedError),
    ChargeNotSet(rc::err::basic::ChargeFoundError),
    UnremovableAutocharge(rc::err::basic::ItemKindRemoveError),
    InvalidSkillLevel(rc::err::basic::SkillLevelError),
    InvalidFighterCount(rc::err::basic::FighterCountError),
    ProjecteeCantTakeProjs(rc::err::basic::ItemReceiveProjError),
    ProjectionNotFound(rc::err::basic::ProjFoundError),
    ProjectionAlreadyExists(rc::err::basic::ProjNotFoundError),
    // Misc
    InvalidSecStatus(rc::err::basic::SecStatusError),
    InvalidDpsProfileEm(rc::err::basic::EmDmgError),
    InvalidDpsProfileTherm(rc::err::basic::ThermDmgError),
    InvalidDpsProfileKin(rc::err::basic::KinDmgError),
    InvalidDpsProfileExpl(rc::err::basic::ExplDmgError),
    InvalidBreacherAbs(rc::err::basic::BreacherAbsDmgError),
    InvalidBreacherRel(rc::err::basic::BreacherRelDmgError),
}
impl std::error::Error for HExecError {}
impl std::fmt::Display for HExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // Fits
            HExecError::FitNotFoundPrimary(e) => write!(f, "{e}"),
            HExecError::FitNotFoundSecondary(e) => write!(f, "{e}"),
            HExecError::FitCharacterNotFound(fit_id) => write!(f, "fit {fit_id} has no character set"),
            HExecError::FitShipNotFound(fit_id) => write!(f, "fit {fit_id} has no ship set"),
            HExecError::FitStanceNotFound(fit_id) => write!(f, "fit {fit_id} has no stance set"),
            HExecError::FitIsNotInFleet(e) => write!(f, "{e}"),
            HExecError::FitIsNotInThisFleet(fleet_id, fit_id) => write!(f, "fit {fit_id} is not in fleet {fleet_id}"),
            HExecError::FitItemKindNotFound(e) => write!(f, "{e}"),
            // Fleets
            HExecError::FleetNotFoundPrimary(e) => write!(f, "{e}"),
            HExecError::FleetNotFoundSecondary(e) => write!(f, "{e}"),
            // Items
            HExecError::ItemNotFoundPrimary(e) => write!(f, "{e}"),
            HExecError::ItemNotFoundSecondary(e) => write!(f, "{e}"),
            HExecError::ItemKindMismatch(e) => write!(f, "{e}"),
            HExecError::SkillIdCollision(e) => write!(f, "{e}"),
            HExecError::MutationNotSet(e) => write!(f, "{e}"),
            HExecError::ChargeNotSet(e) => write!(f, "{e}"),
            HExecError::UnremovableAutocharge(e) => write!(f, "{e}"),
            HExecError::InvalidSkillLevel(e) => write!(f, "{e}"),
            HExecError::InvalidFighterCount(e) => write!(f, "{e}"),
            HExecError::ProjecteeCantTakeProjs(e) => write!(f, "{e}"),
            HExecError::ProjectionNotFound(e) => write!(f, "{e}"),
            HExecError::ProjectionAlreadyExists(e) => write!(f, "{e}"),
            // Misc
            HExecError::InvalidSecStatus(e) => write!(f, "{e}"),
            HExecError::InvalidDpsProfileEm(e) => write!(f, "{e}"),
            HExecError::InvalidDpsProfileTherm(e) => write!(f, "{e}"),
            HExecError::InvalidDpsProfileKin(e) => write!(f, "{e}"),
            HExecError::InvalidDpsProfileExpl(e) => write!(f, "{e}"),
            HExecError::InvalidBreacherAbs(e) => write!(f, "{e}"),
            HExecError::InvalidBreacherRel(e) => write!(f, "{e}"),
        }
    }
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
            HExecError::InvalidDpsProfileEm(_) => "EXC-024",
            HExecError::InvalidDpsProfileTherm(_) => "EXC-025",
            HExecError::InvalidDpsProfileKin(_) => "EXC-026",
            HExecError::InvalidDpsProfileExpl(_) => "EXC-027",
            HExecError::InvalidBreacherAbs(_) => "EXC-029",
            HExecError::InvalidBreacherRel(_) => "EXC-030",
        }
        .to_string()
    }
}
