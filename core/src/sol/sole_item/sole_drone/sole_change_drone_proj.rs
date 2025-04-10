use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ProjFoundError},
    sol::{AttrVal, ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn change_drone_proj(
        &mut self,
        item_id: &ItemId,
        projectee_item_id: &ItemId,
        range: Option<AttrVal>,
    ) -> Result<(), ChangeDroneProjError> {
        let item_key = self
            .uad
            .items
            .key_by_id_err(item_id)
            .map_err(ChangeDroneProjError::ProjectorNotFound)?;
        let projectee_item_key = self
            .uad
            .items
            .key_by_id_err(projectee_item_id)
            .map_err(ChangeDroneProjError::ProjecteeNotFound)?;
        self.change_drone_proj_internal(item_key, projectee_item_key, range)
    }
    pub(in crate::sol) fn change_drone_proj_internal(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) -> Result<(), ChangeDroneProjError> {
        // Check if projection is defined before changing it
        let drone = self.uad.items.get_mut(item_key).get_drone_mut()?;
        let old_range = match drone.get_projs().get(&projectee_item_key) {
            Some(old_range) => *old_range,
            None => {
                return Err(ProjFoundError {
                    projector_item_id: drone.get_item_id(),
                    projectee_item_id: self.uad.items.id_by_key(projectee_item_key),
                }
                .into());
            }
        };
        // Do nothing if ranges are equal
        if range == old_range {
            return Ok(());
        }
        // Update user data
        drone.get_projs_mut().add(projectee_item_key, range);
        // Update services
        self.change_item_key_projection_range_in_svc(item_key, projectee_item_key, range);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeDroneProjError {
    #[error("{0}")]
    ProjectorNotFound(#[source] ItemFoundError),
    #[error("{0}")]
    ProjectorIsNotDrone(#[from] ItemKindMatchError),
    #[error("{0}")]
    ProjecteeNotFound(#[source] ItemFoundError),
    #[error("{0}")]
    ProjectionNotFound(#[from] ProjFoundError),
}
