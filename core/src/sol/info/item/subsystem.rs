use crate::{
    defs::{EItemId, SlotIndex, SolFitId, SolItemId},
    sol::uad::item::SolSubsystem,
};

pub struct SolSubsystemInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub slot: Option<SlotIndex>,
    pub enabled: bool,
}
impl SolSubsystemInfo {
    fn new(id: SolItemId, type_id: EItemId, fit_id: SolFitId, slot: Option<SlotIndex>, enabled: bool) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            slot,
            enabled,
        }
    }
}
impl From<&SolSubsystem> for SolSubsystemInfo {
    fn from(sol_subsystem: &SolSubsystem) -> Self {
        SolSubsystemInfo::new(
            sol_subsystem.get_id(),
            sol_subsystem.get_type_id(),
            sol_subsystem.get_fit_id(),
            sol_subsystem.get_slot(),
            sol_subsystem.get_subsystem_state(),
        )
    }
}
