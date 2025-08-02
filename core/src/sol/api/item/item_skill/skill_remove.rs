use crate::{
    sol::{SolarSystem, api::SkillMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_skill(
        &mut self,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        SolarSystem::util_remove_skill(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
        let u_skill = self.u_data.items.get(item_key).get_skill().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_skill.get_fit_key());
        u_fit.skills.remove(&u_skill.get_type_id());
        self.u_data.items.remove(item_key);
    }
}

impl<'a> SkillMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_skill(self.key, &mut reuse_eupdates);
    }
}
