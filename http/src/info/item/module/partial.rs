use crate::{
    info::{item::charge::HChargeInfo, HItemInfoMode},
    shared::{HModRack, HState},
};

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HModuleInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    pub(crate) type_id: rc::EItemId,
    pub(crate) state: HState,
    pub(crate) rack: HModRack,
    pub(crate) pos: rc::Idx,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) charge: Option<HChargeInfo>,
    #[serde_as(as = "Vec<(serde_with::DisplayFromStr, _)>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) tgts: Vec<(rc::SolItemId, Option<rc::AttrVal>)>,
}
impl HModuleInfoPartial {
    pub(super) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_module_info: &rc::SolModuleInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self {
            id: core_module_info.id,
            fit_id: core_module_info.fit_id,
            type_id: core_module_info.a_item_id,
            state: (&core_module_info.state).into(),
            rack: (&core_module_info.rack).into(),
            pos: core_module_info.pos,
            charge: core_module_info
                .charge_info
                .as_ref()
                .map(|v| HChargeInfo::mk_info(core_sol, v, item_mode)),
            tgts: core_module_info.tgts.iter().map(|v| (v.item_id, v.range)).collect(),
        }
    }
}
