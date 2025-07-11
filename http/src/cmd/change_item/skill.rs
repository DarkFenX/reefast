use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, apply_effect_modes},
    },
    shared::HSkillLevel,
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeSkillCmd {
    #[serde(default)]
    type_id: Option<rc::ItemTypeId>,
    #[serde(default)]
    level: Option<HSkillLevel>,
    #[serde(default)]
    state: Option<bool>,
    #[serde(default)]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeSkillCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_skill = core_sol.get_skill_mut(item_id).map_err(|error| match error {
            rc::err::GetSkillError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetSkillError::ItemIsNotSkill(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            core_skill.set_type_id(type_id).map_err(|error| match error {
                rc::err::SetSkillTypeIdError::SkillIdCollision(e) => HExecError::SkillIdCollision(e),
            })?;
        }
        if let Some(h_level) = self.level {
            let core_level = rc::SkillLevel::new_clamped(h_level);
            core_skill.set_level(core_level);
        }
        if let Some(state) = self.state {
            core_skill.set_state(state);
        }
        apply_effect_modes(&mut core_skill, &self.effect_modes);
        Ok(core_skill.into())
    }
}
