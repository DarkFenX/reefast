#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum CmdResp {
    NoData,
    ItemIds(ItemIdsResp),
}

#[derive(serde::Serialize)]
pub(crate) struct ItemIdsResp {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_id: Option<String>,
}
impl ItemIdsResp {
    fn new(item_id: reefast::ReeId, charge_info: Option<reefast::ChargeInfo>) -> Self {
        Self {
            id: item_id.to_string(),
            charge_id: charge_info.map(|v| v.id.to_string()),
        }
    }
}
impl From<reefast::ImplantInfo> for ItemIdsResp {
    fn from(value: reefast::ImplantInfo) -> Self {
        ItemIdsResp::new(value.id, None)
    }
}
impl From<reefast::ShipInfo> for ItemIdsResp {
    fn from(value: reefast::ShipInfo) -> Self {
        ItemIdsResp::new(value.id, None)
    }
}
impl From<reefast::ModuleInfo> for ItemIdsResp {
    fn from(value: reefast::ModuleInfo) -> Self {
        ItemIdsResp::new(value.id, value.charge_info)
    }
}
impl From<reefast::RigInfo> for ItemIdsResp {
    fn from(value: reefast::RigInfo) -> Self {
        ItemIdsResp::new(value.id, None)
    }
}
