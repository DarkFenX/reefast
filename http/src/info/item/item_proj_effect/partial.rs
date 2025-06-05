use rc::ItemCommon;

use crate::info::item::proj::HProjInfo;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HProjEffectInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: rc::ItemTypeId,
    enabled: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    projs: Vec<HProjInfo>,
}
impl From<&mut rc::ProjEffectMut<'_>> for HProjEffectInfoPartial {
    fn from(core_proj_effect: &mut rc::ProjEffectMut) -> Self {
        Self {
            id: core_proj_effect.get_item_id(),
            kind: "proj_effect",
            type_id: core_proj_effect.get_type_id(),
            enabled: core_proj_effect.get_state(),
            projs: core_proj_effect
                .iter_projs()
                .map(|core_proj| core_proj.into())
                .collect(),
        }
    }
}
