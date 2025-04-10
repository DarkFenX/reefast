use std::collections::HashMap;

use crate::{
    info::{HItemInfoMode, item::autocharge::HAutochargeInfo},
    shared::{HEffectId, HMinionState},
};

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFighterInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::FitId,
    pub(crate) state: HMinionState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) count: Option<(rc::Count, rc::Count)>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(crate) autocharges: HashMap<HEffectId, HAutochargeInfo>,
    #[serde_as(as = "Vec<(serde_with::DisplayFromStr, _)>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) projs: Vec<(rc::ItemId, Option<rc::AttrVal>)>,
}
impl HFighterInfoPartial {
    pub(super) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_fighter_info: &rc::FighterInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self {
            id: core_fighter_info.id,
            kind: "fighter",
            type_id: core_fighter_info.type_id,
            fit_id: core_fighter_info.fit_id,
            state: (&core_fighter_info.state).into(),
            count: core_fighter_info.count.as_ref().map(|v| (v.current, v.max)),
            autocharges: core_fighter_info
                .autocharges
                .iter()
                .map(|(k, v)| (k.into(), HAutochargeInfo::mk_info(core_sol, v, item_mode)))
                .collect(),
            projs: core_fighter_info.projs.iter().map(|v| (v.item_id, v.range)).collect(),
        }
    }
}
