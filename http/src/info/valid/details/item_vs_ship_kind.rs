use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValItemVsShipKindFail {
    ship_kind: HShipKind,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    items: HashMap<rc::ItemId, HShipKind>,
}
impl From<&rc::val::ValItemVsShipKindFail> for HValItemVsShipKindFail {
    fn from(core_val_fail: &rc::val::ValItemVsShipKindFail) -> Self {
        Self {
            ship_kind: (&core_val_fail.ship_kind).into(),
            items: core_val_fail
                .items
                .iter()
                .map(|v| (v.item_id, (&v.needed_kind).into()))
                .collect(),
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::info::valid) enum HShipKind {
    Ship,
    Structure,
    Unknown,
}
impl From<&rc::val::ValShipKind> for HShipKind {
    fn from(core_ship_kind: &rc::val::ValShipKind) -> Self {
        match core_ship_kind {
            rc::val::ValShipKind::Ship => Self::Ship,
            rc::val::ValShipKind::Structure => Self::Structure,
            rc::val::ValShipKind::Unknown => Self::Unknown,
        }
    }
}
