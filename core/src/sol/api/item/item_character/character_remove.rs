use crate::{
    sol::{SolarSystem, api::CharacterMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_character(
        &mut self,
        character_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        SolarSystem::util_remove_character(&mut self.u_data, &mut self.svc, character_key, reuse_eupdates);
        let u_character = self.u_data.items.get(character_key).get_character().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_character.get_fit_key());
        u_fit.character = None;
        self.u_data.items.remove(character_key);
    }
}

impl<'a> CharacterMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_character(self.key, &mut reuse_eupdates);
    }
}
