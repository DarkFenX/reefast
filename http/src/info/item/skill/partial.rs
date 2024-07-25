#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSkillInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::EItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    pub(crate) level: rc::SkillLevel,
    pub(crate) enabled: bool,
}
impl From<&rc::SolSkillInfo> for HSkillInfoPartial {
    fn from(core_skill_info: &rc::SolSkillInfo) -> Self {
        Self {
            id: core_skill_info.id,
            kind: "skill",
            type_id: core_skill_info.a_item_id,
            fit_id: core_skill_info.fit_id,
            level: core_skill_info.level,
            enabled: core_skill_info.enabled,
        }
    }
}
