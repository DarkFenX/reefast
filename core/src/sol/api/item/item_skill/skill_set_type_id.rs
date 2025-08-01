use crate::{
    ad,
    def::ItemTypeId,
    err::basic::SkillEveTypeError,
    sol::{SolarSystem, api::SkillMut},
    ud::{UEffectUpdates, UItemKey},
    util::GetId,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_skill_a_item_id(
        &mut self,
        item_key: UItemKey,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<(), SkillEveTypeError> {
        let u_item = self.u_data.items.get(item_key);
        let old_a_item_id = u_item.get_type_id();
        if old_a_item_id == a_item_id {
            return Ok(());
        }
        let fit_key = u_item.get_skill().unwrap().get_fit_key();
        // Check for collisions before doing anything
        let u_fit = self.u_data.fits.get(fit_key);
        if let Some(fit_skill) = u_fit.skills.get(&a_item_id) {
            return Err(SkillEveTypeError {
                type_id: a_item_id,
                fit_id: u_fit.get_id(),
                item_id: self.u_data.items.id_by_key(fit_skill.item_key),
            });
        }
        // Unload skill
        SolarSystem::util_remove_skill(&self.u_data, &mut self.svc, item_key, u_item, reuse_eupdates);
        // Update adapted item ID and reload adapted data
        self.u_data
            .items
            .get_mut(item_key)
            .get_skill_mut()
            .unwrap()
            .set_type_id(a_item_id, reuse_eupdates, &self.u_data.src);
        // Update fit skill map
        let u_fit = self.u_data.fits.get_mut(fit_key);
        let fit_skill = u_fit.skills.remove(&old_a_item_id).unwrap();
        u_fit.skills.insert(a_item_id, fit_skill);
        // Load skill
        SolarSystem::util_add_skill(&self.u_data, &mut self.svc, item_key, reuse_eupdates);
        Ok(())
    }
}

impl<'a> SkillMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) -> Result<(), SetSkillTypeIdError> {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_skill_a_item_id(self.key, type_id, &mut reuse_eupdates)?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetSkillTypeIdError {
    #[error("{0}")]
    SkillIdCollision(#[from] SkillEveTypeError),
}
