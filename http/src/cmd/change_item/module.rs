use crate::{
    cmd::{
        HCmdResp,
        shared::{HEffectModeMap, HMutationOnChange, HProjDef, HProjDefFull, apply_effect_modes},
    },
    shared::HModuleState,
    util::{HExecError, TriStateField},
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    state: Option<HModuleState>,
    #[serde(default)]
    mutation: TriStateField<HMutationOnChange>,
    #[serde(default)]
    charge: TriStateField<rc::ItemTypeId>,
    #[serde(default)]
    add_projs: Vec<HProjDef>,
    #[serde(default)]
    change_projs: Vec<HProjDefFull>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    rm_projs: Vec<rc::ItemId>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeModuleCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HCmdResp, HExecError> {
        if let Some(state) = &self.state {
            if let Err(error) = core_sol.set_module_state(item_id, state.into()) {
                return Err(match error {
                    rc::err::SetModuleStateError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::SetModuleStateError::ItemIsNotModule(e) => HExecError::ItemKindMismatch(e),
                });
            };
        }
        match &self.mutation {
            TriStateField::Value(mutation) => match mutation {
                HMutationOnChange::AddShort(mutator_id) => {
                    // Remove old mutation if we had any, ignore any errors on the way
                    let _ = core_sol.remove_module_mutation(item_id);
                    let mutation = rc::ItemAddMutation::new(*mutator_id);
                    if let Err(error) = core_sol.add_module_mutation(item_id, mutation) {
                        match error {
                            rc::err::AddModuleMutationError::ItemNotFound(e) => {
                                return Err(HExecError::ItemNotFoundPrimary(e));
                            }
                            rc::err::AddModuleMutationError::ItemIsNotModule(e) => {
                                return Err(HExecError::ItemKindMismatch(e));
                            }
                            rc::err::AddModuleMutationError::MutationAlreadySet(_) => {
                                panic!("no mutation should be set")
                            }
                        };
                    }
                }
                HMutationOnChange::AddFull(mutation) => {
                    // Remove old mutation if we had any, ignore any errors on the way
                    let _ = core_sol.remove_module_mutation(item_id);
                    if let Err(error) = core_sol.add_module_mutation(item_id, mutation.into()) {
                        match error {
                            rc::err::AddModuleMutationError::ItemNotFound(e) => {
                                return Err(HExecError::ItemNotFoundPrimary(e));
                            }
                            rc::err::AddModuleMutationError::ItemIsNotModule(e) => {
                                return Err(HExecError::ItemKindMismatch(e));
                            }
                            rc::err::AddModuleMutationError::MutationAlreadySet(_) => {
                                panic!("no mutation should be set")
                            }
                        };
                    }
                }
                HMutationOnChange::ChangeAttrs(attr_mutations) => {
                    let attr_mutations = attr_mutations
                        .iter()
                        .map(|(k, v)| rc::ItemChangeAttrMutation::new(*k, v.as_ref().map(|v| v.into())))
                        .collect();
                    if let Err(error) = core_sol.change_module_mutation(item_id, attr_mutations) {
                        return Err(match error {
                            rc::err::ChangeModuleMutationError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                            rc::err::ChangeModuleMutationError::ItemIsNotModule(e) => HExecError::ItemKindMismatch(e),
                            rc::err::ChangeModuleMutationError::MutationNotSet(e) => HExecError::MutationNotSet(e),
                        });
                    }
                }
            },
            TriStateField::None => {
                if let Err(error) = core_sol.remove_module_mutation(item_id) {
                    match error {
                        rc::err::RemoveModuleMutationError::ItemNotFound(e) => {
                            return Err(HExecError::ItemNotFoundPrimary(e));
                        }
                        rc::err::RemoveModuleMutationError::ItemIsNotModule(e) => {
                            return Err(HExecError::ItemKindMismatch(e));
                        }
                        // Do nothing if mutation was not there
                        rc::err::RemoveModuleMutationError::MutationNotSet(_) => (),
                    };
                };
            }
            TriStateField::Absent => (),
        }
        match &self.charge {
            TriStateField::Value(charge) => {
                if let Err(error) = core_sol.set_module_charge(item_id, *charge) {
                    return Err(match error {
                        rc::err::SetModuleChargeError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                        rc::err::SetModuleChargeError::ItemIsNotModule(e) => HExecError::ItemKindMismatch(e),
                    });
                }
            }
            TriStateField::None => {
                if let Err(error) = core_sol.remove_module_charge(item_id) {
                    return Err(match error {
                        rc::err::RemoveModuleChargeError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                        rc::err::RemoveModuleChargeError::ItemIsNotModule(e) => HExecError::ItemKindMismatch(e),
                        rc::err::RemoveModuleChargeError::ChargeNotSet(e) => HExecError::ChargeNotSet(e),
                    });
                };
            }
            TriStateField::Absent => (),
        }
        for proj_def in self.add_projs.iter() {
            if let Err(error) = core_sol.add_module_proj(item_id, &proj_def.get_item_id(), proj_def.get_range()) {
                return Err(match error {
                    rc::err::AddModuleProjError::ProjectorNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::AddModuleProjError::ProjectorIsNotModule(e) => HExecError::ItemKindMismatch(e),
                    rc::err::AddModuleProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::AddModuleProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                    rc::err::AddModuleProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
                });
            }
        }
        for proj_def in self.change_projs.iter() {
            if let Err(error) = core_sol.change_module_proj(item_id, &proj_def.get_item_id(), proj_def.get_range()) {
                return Err(match error {
                    rc::err::ChangeModuleProjError::ProjectorNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::ChangeModuleProjError::ProjectorIsNotModule(e) => HExecError::ItemKindMismatch(e),
                    rc::err::ChangeModuleProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::ChangeModuleProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                });
            }
        }
        for projectee_item_id in self.rm_projs.iter() {
            if let Err(error) = core_sol.remove_module_proj(item_id, projectee_item_id) {
                return Err(match error {
                    rc::err::RemoveModuleProjError::ProjectorNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::RemoveModuleProjError::ProjectorIsNotModule(e) => HExecError::ItemKindMismatch(e),
                    rc::err::RemoveModuleProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::RemoveModuleProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                });
            }
        }
        apply_effect_modes(core_sol, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
