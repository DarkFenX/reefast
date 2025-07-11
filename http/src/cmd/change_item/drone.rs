use crate::{
    cmd::{
        HItemIdsResp,
        shared::{
            HEffectModeMap, HMutationOnChange, HProjDef, HProjDefFull, apply_effect_modes, apply_mattrs_on_add,
            apply_mattrs_on_change,
        },
    },
    shared::HMinionState,
    util::{HExecError, TriStateField},
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeDroneCmd {
    #[serde(default)]
    type_id: Option<rc::ItemTypeId>,
    #[serde(default)]
    state: Option<HMinionState>,
    #[serde(default)]
    mutation: TriStateField<HMutationOnChange>,
    #[serde(default)]
    add_projs: Vec<HProjDef>,
    #[serde(default)]
    change_projs: Vec<HProjDefFull>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    rm_projs: Vec<rc::ItemId>,
    #[serde(default)]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeDroneCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_drone = core_sol.get_drone_mut(item_id).map_err(|error| match error {
            rc::err::GetDroneError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetDroneError::ItemIsNotDrone(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            core_drone.set_type_id(type_id);
        }
        if let Some(state) = &self.state {
            core_drone.set_state(state.into());
        }
        match &self.mutation {
            TriStateField::Value(mutation) => match mutation {
                // Mutates item or updates existing mutation
                HMutationOnChange::Mutator(mutator_id) => {
                    match core_drone.get_mutation_mut() {
                        Some(mutation) => mutation.set_mutator_id(*mutator_id),
                        None => core_drone.mutate(*mutator_id).unwrap(),
                    };
                }
                // Updates existing mutation
                HMutationOnChange::Attrs(h_attr_mutations) => {
                    let core_mutation = match core_drone.get_mutation_mut() {
                        Some(core_mutation) => core_mutation,
                        None => return Err(HExecError::MutationNotSet(*item_id)),
                    };
                    apply_mattrs_on_change(core_mutation, h_attr_mutations);
                }
                // Mutates item, or overwrites mutation, if it was set
                HMutationOnChange::MutatorAndAttrs(mutation) => {
                    if let Some(core_mutation) = core_drone.get_mutation_mut() {
                        core_mutation.remove();
                    }
                    let core_mutation = core_drone.mutate(mutation.mutator_id).unwrap();
                    apply_mattrs_on_add(core_mutation, mutation);
                }
            },
            TriStateField::None => {
                // Do nothing if mutation was not there
                if let Some(core_mutation) = core_drone.get_mutation_mut() {
                    core_mutation.remove();
                }
            }
            TriStateField::Absent => (),
        }
        for proj_def in self.add_projs.iter() {
            core_drone
                .add_proj(&proj_def.get_item_id(), proj_def.get_range().into())
                .map_err(|error| match error {
                    rc::err::AddRangedProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::AddRangedProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                    rc::err::AddRangedProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
                })?;
        }
        for proj_def in self.change_projs.iter() {
            core_drone
                .get_proj_mut(&proj_def.get_item_id())
                .map_err(|error| match error {
                    rc::err::GetRangedProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::GetRangedProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?
                .set_range(proj_def.get_range().into());
        }
        for projectee_item_id in self.rm_projs.iter() {
            core_drone
                .get_proj_mut(projectee_item_id)
                .map_err(|error| match error {
                    rc::err::GetRangedProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::GetRangedProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?
                .remove();
        }
        apply_effect_modes(&mut core_drone, &self.effect_modes);
        Ok(core_drone.into())
    }
}
