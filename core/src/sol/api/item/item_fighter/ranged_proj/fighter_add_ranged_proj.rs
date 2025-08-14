use itertools::Itertools;

use crate::{
    def::ItemId,
    err::basic::{ItemReceiveProjError, ProjNotFoundError},
    sol::{
        SolarSystem,
        api::{AddProjError, FighterMut, ProjMut},
    },
    ud::{UItemKey, UProjData},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_fighter_proj(
        &mut self,
        item_key: UItemKey,
        projectee_key: UItemKey,
    ) -> Result<(), AddProjError> {
        // Check projector
        let u_fighter = self.u_data.items.get(item_key).get_fighter().unwrap();
        // Check if projection has already been defined
        let projectee_u_item = self.u_data.items.get(projectee_key);
        if u_fighter.get_projs().contains(&projectee_key) {
            return Err(ProjNotFoundError {
                projector_item_id: u_fighter.get_item_id(),
                projectee_item_id: projectee_u_item.get_item_id(),
            }
            .into());
        }
        // Check if projectee can receive projections by getting its position in space
        let projectee_pos = match projectee_u_item.get_pos() {
            Some(projectee_pos) => *projectee_pos,
            None => {
                return Err(ItemReceiveProjError {
                    item_id: projectee_u_item.get_item_id(),
                    item_kind: projectee_u_item.get_name(),
                }
                .into());
            }
        };
        let fighter_pos = *u_fighter.get_pos();
        let u_proj_data = Some(UProjData::from_positions_with_axt(
            fighter_pos,
            projectee_pos,
            u_fighter.get_axt(),
            projectee_u_item.get_axt(),
        ));
        let autocharge_keys = u_fighter.get_autocharges().values().collect_vec();
        // Update user data for fighter
        let u_fighter = self.u_data.items.get_mut(item_key).get_fighter_mut().unwrap();
        u_fighter.get_projs_mut().add(projectee_key, u_proj_data);
        self.rev_projs.reg_projectee(item_key, projectee_key);
        // Update services for fighte
        SolarSystem::util_add_item_projection(&self.u_data, &mut self.svc, item_key, projectee_key, u_proj_data);
        for autocharge_key in autocharge_keys {
            // Update user data for autocharge
            let u_autocharge = self.u_data.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
            u_autocharge.get_projs_mut().add(projectee_key, u_proj_data);
            self.rev_projs.reg_projectee(autocharge_key, projectee_key);
            // Update services for autocharge
            SolarSystem::util_add_item_projection(
                &self.u_data,
                &mut self.svc,
                autocharge_key,
                projectee_key,
                u_proj_data,
            );
        }
        Ok(())
    }
}

impl<'a> FighterMut<'a> {
    pub fn add_proj(&mut self, projectee_item_id: &ItemId) -> Result<ProjMut<'_>, AddProjError> {
        let projectee_key = self.sol.u_data.items.key_by_id_err(projectee_item_id)?;
        self.sol.internal_add_fighter_proj(self.key, projectee_key)?;
        Ok(ProjMut::new(self.sol, self.key, projectee_key))
    }
}
