pub mod basic;
pub use crate::{
    sol::{
        sole_calc::{
            GetItemAttrError, IterItemAttrsError, IterItemEffectsError, IterItemModifiersError, SetItemEffectModeError,
            SetItemEffectModesError,
        },
        sole_dmg_profile::{
            GetFitRahIncomingDmgError, RemoveFitRahIncomingDmgError, SetDefaultIncomingDmgError,
            SetFitRahIncomingDmgError,
        },
        sole_fit::{GetFitError, RemoveFitError, SetFitFleetError, UnsetFitFleetError},
        sole_fleet::{GetFleetError, RemoveFleetError},
        sole_item::{
            AddBoosterError, AddDroneError, AddDroneMutationError, AddDroneProjError, AddFighterError,
            AddFighterProjError, AddFwEffectError, AddImplantError, AddModuleError, AddModuleMutationError,
            AddModuleProjError, AddProjEffectProjError, AddRigError, AddServiceError, AddSkillError, AddSubsystemError,
            ChangeDroneMutationError, ChangeDroneProjError, ChangeFighterProjError, ChangeModuleMutationError,
            ChangeModuleProjError, GetAutochargeError, GetBoosterError, GetChargeError, GetDroneError, GetFighterError,
            GetFitBoostersError, GetFitCharacterError, GetFitDronesError, GetFitFightersError, GetFitFwEffectsError,
            GetFitImplantsError, GetFitModulesError, GetFitRigsError, GetFitServicesError, GetFitShipError,
            GetFitSkillsError, GetFitStanceError, GetFitSubsystemsError, GetFwEffectError, GetImplantError,
            GetItemError, GetModuleError, GetProjEffectError, GetRigError, GetServiceError, GetSkillError,
            GetSubsystemError, GetSwEffectError, RemoveBoosterError, RemoveCharacterError, RemoveChargeError,
            RemoveDroneError, RemoveDroneMutationError, RemoveDroneProjError, RemoveFighterCountOverrideError,
            RemoveFighterError, RemoveFighterProjError, RemoveFitCharacterError, RemoveFitShipError,
            RemoveFitStanceError, RemoveFwEffectError, RemoveImplantError, RemoveItemError, RemoveModuleChargeError,
            RemoveModuleError, RemoveModuleMutationError, RemoveModuleProjError, RemoveProjEffectError,
            RemoveProjEffectProjError, RemoveRigError, RemoveServiceError, RemoveShipError, RemoveSkillError,
            RemoveStanceError, RemoveSubsystemError, RemoveSwEffectError, SetAutochargeStateError,
            SetBoosterSideEffectStateError, SetBoosterStateError, SetCharacterStateError, SetChargeStateError,
            SetDroneStateError, SetFighterCountOverrideError, SetFighterStateError, SetFitCharacterError,
            SetFitCharacterStateError, SetFitShipError, SetFitShipStateError, SetFitStanceError,
            SetFitStanceStateError, SetFwEffectStateError, SetImplantStateError, SetModuleChargeError,
            SetModuleStateError, SetProjEffectStateError, SetRigStateError, SetServiceStateError, SetShipStateError,
            SetSkillLevelError, SetSkillStateError, SetStanceStateError, SetSubsystemStateError, SetSwEffectStateError,
        },
        sole_sec::{GetFitSecStatusError, SetFitSecStatusError},
        sole_vast::{TryFitItemsError, ValidateFitError},
    },
    src::SrcInitError,
};
