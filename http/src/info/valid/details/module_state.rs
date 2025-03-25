use std::collections::HashMap;

use crate::shared::HModuleState;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HValModuleStateFail {
    #[serde(flatten)]
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    data: HashMap<rc::ItemId, HValModuleStateItemInfo>,
}
impl HValModuleStateFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::val::ValModuleStateFail>> for HValModuleStateFail {
    fn from(core_val_fails: &Vec<rc::val::ValModuleStateFail>) -> Self {
        Self {
            data: core_val_fails.iter().map(|v| (v.item_id, v.into())).collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValModuleStateItemInfo {
    state: HModuleState,
    max_state: HModuleState,
}
impl From<&rc::val::ValModuleStateFail> for HValModuleStateItemInfo {
    fn from(core_val_fail: &rc::val::ValModuleStateFail) -> Self {
        Self {
            state: (&core_val_fail.state).into(),
            max_state: (&core_val_fail.max_state).into(),
        }
    }
}
