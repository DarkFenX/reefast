use itertools::Itertools;

use crate::{
    defs::{AttrVal, SolItemId},
    err::basic::{ItemFoundError, ItemKindMatchError, ItemReceiveProjError, ProjNotFoundError},
    sol::{SolView, SolarSystem},
};

impl SolarSystem {
    pub fn add_fighter_proj(
        &mut self,
        item_id: &SolItemId,
        projectee_item_id: SolItemId,
        range: Option<AttrVal>,
    ) -> Result<(), AddFighterProjError> {
        // Check projector
        let fighter = self
            .items
            .get_item(item_id)
            .map_err(|e| AddFighterProjError::ProjectorNotFound(e))?
            .get_fighter()
            .map_err(|e| AddFighterProjError::ProjectorIsNotFighter(e))?;
        // Check if projection has already been defined
        if fighter.get_projs().contains(&projectee_item_id) {
            return Err(AddFighterProjError::ProjectionAlreadyExists(ProjNotFoundError::new(
                *item_id,
                projectee_item_id,
            )));
        }
        // Check if projectee can receive projections
        let projectee_item = self
            .items
            .get_item(&projectee_item_id)
            .map_err(|e| AddFighterProjError::ProjecteeNotFound(e))?;
        if !projectee_item.can_receive_projs() {
            return Err(AddFighterProjError::ProjecteeCantTakeProjs(ItemReceiveProjError::new(
                projectee_item_id,
                projectee_item.get_name(),
            )));
        }
        // Update skeleton for fighter
        let fighter = self.items.get_item_mut(item_id).unwrap().get_fighter_mut().unwrap();
        let autocharge_ids = fighter.get_autocharges().values().map(|v| *v).collect_vec();
        fighter.get_projs_mut().add(projectee_item_id, range);
        self.proj_tracker.reg_projectee(*item_id, projectee_item_id);
        // Update services for fighter
        let fighter_item = self.items.get_item(item_id).unwrap();
        let projectee_item = self.items.get_item(&projectee_item_id).unwrap();
        self.svcs.add_item_projection(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            fighter_item,
            projectee_item,
            range,
        );
        for autocharge_id in autocharge_ids {
            // Update skeleton for autocharge
            let autocharge = self
                .items
                .get_item_mut(&autocharge_id)
                .unwrap()
                .get_charge_mut()
                .unwrap();
            autocharge.get_projs_mut().add(projectee_item_id, range);
            self.proj_tracker.reg_projectee(autocharge_id, projectee_item_id);
            // Update services for autocharge
            let autocharge_item = self.items.get_item(&autocharge_id).unwrap();
            let projectee_item = self.items.get_item(&projectee_item_id).unwrap();
            self.svcs.add_item_projection(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                autocharge_item,
                projectee_item,
                range,
            );
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum AddFighterProjError {
    ProjectorNotFound(ItemFoundError),
    ProjectorIsNotFighter(ItemKindMatchError),
    ProjecteeNotFound(ItemFoundError),
    ProjecteeCantTakeProjs(ItemReceiveProjError),
    ProjectionAlreadyExists(ProjNotFoundError),
}
impl std::error::Error for AddFighterProjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ProjectorNotFound(e) => Some(e),
            Self::ProjectorIsNotFighter(e) => Some(e),
            Self::ProjecteeNotFound(e) => Some(e),
            Self::ProjecteeCantTakeProjs(e) => Some(e),
            Self::ProjectionAlreadyExists(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddFighterProjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProjectorNotFound(e) => e.fmt(f),
            Self::ProjectorIsNotFighter(e) => e.fmt(f),
            Self::ProjecteeNotFound(e) => e.fmt(f),
            Self::ProjecteeCantTakeProjs(e) => e.fmt(f),
            Self::ProjectionAlreadyExists(e) => e.fmt(f),
        }
    }
}
impl From<ProjNotFoundError> for AddFighterProjError {
    fn from(error: ProjNotFoundError) -> Self {
        Self::ProjectionAlreadyExists(error)
    }
}
