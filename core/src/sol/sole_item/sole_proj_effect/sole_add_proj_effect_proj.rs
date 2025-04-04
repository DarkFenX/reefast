use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ItemReceiveProjError, ProjNotFoundError},
    sol::{ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn add_proj_effect_proj(
        &mut self,
        item_id: &ItemId,
        projectee_item_id: ItemId,
    ) -> Result<(), AddProjEffectProjError> {
        // Check projector
        let proj_effect = self
            .uad
            .items
            .get_item(item_id)
            .map_err(AddProjEffectProjError::ProjectorNotFound)?
            .get_proj_effect()
            .map_err(AddProjEffectProjError::ProjectorIsNotProjEffect)?;
        // Check if projection has already been defined
        if proj_effect.get_projs().contains(&projectee_item_id) {
            return Err(AddProjEffectProjError::ProjectionAlreadyExists(ProjNotFoundError {
                projector_item_id: *item_id,
                projectee_item_id,
            }));
        }
        // Check if projectee can receive projections
        let projectee_item = self
            .uad
            .items
            .get_item(&projectee_item_id)
            .map_err(AddProjEffectProjError::ProjecteeNotFound)?;
        if !projectee_item.can_receive_projs() {
            return Err(AddProjEffectProjError::ProjecteeCantTakeProjs(ItemReceiveProjError {
                item_id: projectee_item_id,
                item_kind: projectee_item.get_name(),
            }));
        }
        // Update user data
        let proj_effect = self
            .uad
            .items
            .get_item_mut(item_id)
            .unwrap()
            .get_proj_effect_mut()
            .unwrap();
        proj_effect.get_projs_mut().add(projectee_item_id, None);
        self.proj_tracker.reg_projectee(*item_id, projectee_item_id);
        // Update services
        self.add_item_id_projection_to_svc(item_id, &projectee_item_id, None);
        Ok(())
    }
}

#[derive(Debug)]
pub enum AddProjEffectProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotProjEffect(ItemKindMatchError),
    ProjecteeNotFound(ItemFoundError),
    ProjecteeCantTakeProjs(ItemReceiveProjError),
    ProjectionAlreadyExists(ProjNotFoundError),
}
impl std::error::Error for AddProjEffectProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotProjEffect(e) => Some(e),
            Self::ProjecteeNotFound(e) => Some(e),
            Self::ProjecteeCantTakeProjs(e) => Some(e),
            Self::ProjectionAlreadyExists(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddProjEffectProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotProjEffect(e) => e.fmt(f),
            Self::ProjecteeNotFound(e) => e.fmt(f),
            Self::ProjecteeCantTakeProjs(e) => e.fmt(f),
            Self::ProjectionAlreadyExists(e) => e.fmt(f),
        }
    }
}
impl From<ProjNotFoundError> for AddProjEffectProjError {
    fn from(error: ProjNotFoundError) -> Self {
        Self::ProjectionAlreadyExists(error)
    }
}
