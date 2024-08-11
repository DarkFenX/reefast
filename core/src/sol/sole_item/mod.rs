//! Solar system extension methods which handle item manipulation.

pub use misc::{SolOrdAddMode, SolOrdRmMode};
pub use sole_autocharge::GetAutochargeError;
pub use sole_booster::{
    AddBoosterError, GetBoosterError, GetFitBoostersError, RemoveBoosterError, SetBoosterSideEffectStateError,
    SetBoosterStateError,
};
pub use sole_character::{
    GetFitCharacterError, RemoveCharacterError, RemoveFitCharacterError, SetCharacterStateError, SetFitCharacterError,
    SetFitCharacterStateError,
};
pub use sole_charge::{GetChargeError, RemoveChargeError};
pub use sole_drone::{AddDroneError, GetDroneError, GetFitDronesError, RemoveDroneError, SetDroneStateError};
pub use sole_fighter::{
    AddFighterError, GetFighterError, GetFitFightersError, RemoveFighterError, SetFighterStateError,
};
pub use sole_fw_effect::{
    AddFwEffectError, GetFitFwEffectsError, GetFwEffectError, RemoveFwEffectError, SetFwEffectStateError,
};
pub use sole_implant::{
    AddImplantError, GetFitImplantsError, GetImplantError, RemoveImplantError, SetImplantStateError,
};
pub use sole_item::{GetItemError, RemoveItemError};
pub use sole_module::{
    AddModuleError, AddModuleProjError, ChangeModuleProjError, GetFitModulesError, GetModuleError,
    RemoveModuleChargeError, RemoveModuleError, RemoveModuleProjError, SetModuleChargeError, SetModuleStateError,
};
pub use sole_proj_effect::{
    AddProjEffectError, AddProjEffectProjError, GetProjEffectError, RemoveProjEffectError, RemoveProjEffectProjError,
    SetProjEffectStateError,
};
pub use sole_rig::{AddRigError, GetFitRigsError, GetRigError, RemoveRigError, SetRigStateError};
pub use sole_ship::{
    GetFitShipError, RemoveFitShipError, RemoveShipError, SetFitShipError, SetFitShipStateError, SetShipStateError,
};
pub use sole_skill::{
    AddSkillError, GetFitSkillsError, GetSkillError, RemoveSkillError, SetSkillLevelError, SetSkillStateError,
};
pub use sole_stance::{
    GetFitStanceError, RemoveFitStanceError, RemoveStanceError, SetFitStanceError, SetFitStanceStateError,
    SetStanceStateError,
};
pub use sole_subsystem::{
    AddSubsystemError, GetFitSubsystemsError, GetSubsystemError, RemoveSubsystemError, SetSubsystemStateError,
};
pub use sole_sw_effect::{AddSwEffectError, GetSwEffectError, RemoveSwEffectError, SetSwEffectStateError};

mod misc;
mod sole_autocharge;
mod sole_booster;
mod sole_character;
mod sole_charge;
mod sole_drone;
mod sole_fighter;
mod sole_fw_effect;
mod sole_implant;
mod sole_item;
mod sole_module;
mod sole_proj_effect;
mod sole_rig;
mod sole_ship;
mod sole_skill;
mod sole_stance;
mod sole_subsystem;
mod sole_sw_effect;
