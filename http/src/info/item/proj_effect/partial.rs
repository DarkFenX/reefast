#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HProjEffectInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    pub(crate) type_id: rc::EItemId,
    pub(crate) enabled: bool,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) tgts: Vec<rc::SolItemId>,
}
impl From<&rc::SolProjEffectInfo> for HProjEffectInfoPartial {
    fn from(core_proj_effect_info: &rc::SolProjEffectInfo) -> Self {
        Self {
            id: core_proj_effect_info.id,
            type_id: core_proj_effect_info.a_item_id,
            enabled: core_proj_effect_info.enabled,
            tgts: core_proj_effect_info.tgts.clone(),
        }
    }
}
