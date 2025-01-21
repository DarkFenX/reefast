use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HResValFail {
    used: rc::AttrVal,
    output: Option<rc::AttrVal>,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    users: HashMap<rc::SolItemId, rc::AttrVal>,
}
impl From<&rc::SolResValFail> for HResValFail {
    fn from(core_val_fail: &rc::SolResValFail) -> Self {
        Self {
            used: core_val_fail.used,
            output: core_val_fail.output,
            users: core_val_fail.users.iter().map(|v| (v.item_id, v.used)).collect(),
        }
    }
}
