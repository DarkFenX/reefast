use crate::shared::HState;

#[derive(serde::Serialize)]
pub(crate) struct HFighterInfoPartial {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::SsItemId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::SsFitId,
    pub(crate) type_id: rc::EItemId,
    pub(crate) state: HState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) amt_override: Option<rc::Amount>,
}
impl From<&rc::SsFighterInfo> for HFighterInfoPartial {
    fn from(core_fighter_info: &rc::SsFighterInfo) -> Self {
        Self {
            id: core_fighter_info.id,
            fit_id: core_fighter_info.fit_id,
            type_id: core_fighter_info.a_item_id,
            state: (&core_fighter_info.state).into(),
            amt_override: core_fighter_info.amt_override,
        }
    }
}
