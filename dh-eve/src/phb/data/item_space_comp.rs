use std::collections::HashMap;

use crate::phb::fsd::{FsdId, FsdMerge};

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PItemSpaceComp {
    #[serde(rename = "systemDbuffEmitter", default)]
    pub(in crate::phb) system_dbuff_emitter: Option<PItemSpaceCompSe>,
    #[serde(rename = "appliedProximityEffects", default)]
    pub(in crate::phb) applied_proximity_effects: Option<PItemSpaceCompPe>,
    #[serde(rename = "proximityTrap", default)]
    pub(in crate::phb) proximity_trap: Option<PItemSpaceCompPt>,
    #[serde(rename = "linkWithShip", default)]
    pub(in crate::phb) link_with_ship: Option<PItemSpaceCompSl>,
}
impl FsdMerge<rc::ed::EItemSpaceComp> for PItemSpaceComp {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EItemSpaceComp> {
        vec![rc::ed::EItemSpaceComp {
            item_id: id,
            system_emitter_buffs: match self.system_dbuff_emitter {
                Some(data) => data
                    .buffs
                    .into_iter()
                    .map(|(id, value)| rc::ed::EItemSpaceCompBuff { id, value })
                    .collect(),
                None => Vec::new(),
            },
            proxy_effect_buffs: match self.applied_proximity_effects {
                Some(data) => data
                    .buffs
                    .into_iter()
                    .map(|(id, value)| rc::ed::EItemSpaceCompBuff { id, value })
                    .collect(),
                None => Vec::new(),
            },
            proxy_trigger_buffs: match self.proximity_trap {
                Some(data) => data
                    .buffs
                    .into_iter()
                    .map(|(id, value)| rc::ed::EItemSpaceCompBuff { id, value })
                    .collect(),
                None => Vec::new(),
            },
            ship_link_buffs: match self.link_with_ship {
                Some(data) => data
                    .buffs
                    .into_iter()
                    .map(|(id, value)| rc::ed::EItemSpaceCompBuff { id, value })
                    .collect(),
                None => Vec::new(),
            },
        }]
    }
}

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PItemSpaceCompSe {
    #[serde(rename = "dbuffCollections", default)]
    pub(in crate::phb) buffs: HashMap<rc::ed::EBuffId, rc::ed::EAttrVal>,
}

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PItemSpaceCompPe {
    #[serde(rename = "effects", default)]
    pub(in crate::phb) buffs: HashMap<rc::ed::EBuffId, rc::ed::EAttrVal>,
}

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PItemSpaceCompPt {
    #[serde(rename = "dbuffs", default)]
    pub(in crate::phb) buffs: HashMap<rc::ed::EBuffId, rc::ed::EAttrVal>,
}

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PItemSpaceCompSl {
    #[serde(rename = "dbuffs", default)]
    pub(in crate::phb) buffs: HashMap<rc::ed::EBuffId, rc::ed::EAttrVal>,
}
