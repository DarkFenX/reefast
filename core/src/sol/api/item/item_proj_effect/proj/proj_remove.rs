use crate::{
    def::ItemKey,
    err::basic::ProjFoundError,
    sol::{SolarSystem, api::ProjMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_proj_effect_proj(
        &mut self,
        item_key: ItemKey,
        projectee_key: ItemKey,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined
        let uad_item = self.uad.items.get(item_key);
        let uad_proj_effect = uad_item.get_proj_effect().unwrap();
        let projectee_uad_item = self.uad.items.get(projectee_key);
        if !uad_proj_effect.get_projs().contains(&projectee_key) {
            return Err(ProjFoundError {
                projector_item_id: uad_proj_effect.get_item_id(),
                projectee_item_id: projectee_uad_item.get_item_id(),
            });
        };
        // Update services
        SolarSystem::util_remove_item_projection(
            &self.uad,
            &mut self.svc,
            item_key,
            uad_item,
            projectee_key,
            projectee_uad_item,
        );
        // Update user data
        self.rprojs.unreg_projectee(&item_key, &projectee_key);
        let uad_proj_effect = self.uad.items.get_mut(item_key).get_proj_effect_mut().unwrap();
        uad_proj_effect.get_projs_mut().remove(&projectee_key);
        Ok(())
    }
}

impl<'a> ProjMut<'a> {
    pub fn remove(self) {
        self.sol
            .internal_remove_proj_effect_proj(self.projector_key, self.projectee_key)
            .unwrap();
    }
}
