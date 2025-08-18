use crate::{
    misc::SkillLevel,
    sol::{SolarSystem, api::SkillMut},
    ud::UItemKey,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_skill_level(&mut self, skill_key: UItemKey, level: SkillLevel) {
        let u_skill = self.u_data.items.get_mut(skill_key).get_skill_mut().unwrap();
        if u_skill.get_level() == level {
            return;
        }
        u_skill.set_level(level.into());
        let fit_skill = self
            .u_data
            .fits
            .get_mut(u_skill.get_fit_key())
            .skills
            .get_mut(&u_skill.get_type_id())
            .unwrap();
        fit_skill.level = level;
        let u_skill = self.u_data.items.get(skill_key).get_skill().unwrap();
        self.svc.notify_skill_level_changed(&self.u_data, skill_key, u_skill);
    }
}

impl<'a> SkillMut<'a> {
    pub fn set_level(&mut self, level: SkillLevel) {
        self.sol.internal_set_skill_level(self.key, level);
    }
}
