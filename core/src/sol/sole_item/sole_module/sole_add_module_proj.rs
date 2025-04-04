use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ItemReceiveProjError, ProjNotFoundError},
    sol::{AttrVal, ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn add_module_proj(
        &mut self,
        item_id: &ItemId,
        projectee_item_id: ItemId,
        range: Option<AttrVal>,
    ) -> Result<(), AddModuleProjError> {
        // Check projector
        let module = self
            .uad
            .items
            .get_item(item_id)
            .map_err(AddModuleProjError::ProjectorNotFound)?
            .get_module()
            .map_err(AddModuleProjError::ProjectorIsNotModule)?;
        // Check if projection has already been defined
        if module.get_projs().contains(&projectee_item_id) {
            return Err(AddModuleProjError::ProjectionAlreadyExists(ProjNotFoundError {
                projector_item_id: *item_id,
                projectee_item_id,
            }));
        }
        // Check if projectee can receive projections
        let projectee_item = self
            .uad
            .items
            .get_item(&projectee_item_id)
            .map_err(AddModuleProjError::ProjecteeNotFound)?;
        if !projectee_item.can_receive_projs() {
            return Err(AddModuleProjError::ProjecteeCantTakeProjs(ItemReceiveProjError {
                item_id: projectee_item_id,
                item_kind: projectee_item.get_name(),
            }));
        }
        // Update user data for module
        let module = self.uad.items.get_item_mut(item_id).unwrap().get_module_mut().unwrap();
        let charge_id = module.get_charge_item_id();
        module.get_projs_mut().add(projectee_item_id, range);
        self.proj_tracker.reg_projectee(*item_id, projectee_item_id);
        // Update services for module
        self.add_item_id_projection_to_svc(item_id, &projectee_item_id, range);
        if let Some(charge_id) = charge_id {
            // Update user data for charge
            let charge = self
                .uad
                .items
                .get_item_mut(&charge_id)
                .unwrap()
                .get_charge_mut()
                .unwrap();
            charge.get_projs_mut().add(projectee_item_id, range);
            self.proj_tracker.reg_projectee(charge_id, projectee_item_id);
            // Update services for charge
            self.add_item_id_projection_to_svc(&charge_id, &projectee_item_id, range);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum AddModuleProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotModule(ItemKindMatchError),
    ProjecteeNotFound(ItemFoundError),
    ProjecteeCantTakeProjs(ItemReceiveProjError),
    ProjectionAlreadyExists(ProjNotFoundError),
}
impl std::error::Error for AddModuleProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotModule(e) => Some(e),
            Self::ProjecteeNotFound(e) => Some(e),
            Self::ProjecteeCantTakeProjs(e) => Some(e),
            Self::ProjectionAlreadyExists(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddModuleProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotModule(e) => e.fmt(f),
            Self::ProjecteeNotFound(e) => e.fmt(f),
            Self::ProjecteeCantTakeProjs(e) => e.fmt(f),
            Self::ProjectionAlreadyExists(e) => e.fmt(f),
        }
    }
}
impl From<ProjNotFoundError> for AddModuleProjError {
    fn from(error: ProjNotFoundError) -> Self {
        Self::ProjectionAlreadyExists(error)
    }
}
