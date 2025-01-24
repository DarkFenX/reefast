#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CItemEffData {
    cd: Option<rc::AttrVal>,
    charge_count: Option<rc::Count>,
    charge_reload_time: Option<rc::AttrVal>,
}
impl From<&rc::ad::AItemEffectData> for CItemEffData {
    fn from(a_item_eff_data: &rc::ad::AItemEffectData) -> Self {
        CItemEffData {
            cd: a_item_eff_data.cd,
            charge_count: a_item_eff_data.charge_count,
            charge_reload_time: a_item_eff_data.charge_reload_time,
        }
    }
}
impl Into<rc::ad::AItemEffectData> for &CItemEffData {
    fn into(self) -> rc::ad::AItemEffectData {
        rc::ad::AItemEffectData {
            cd: self.cd,
            charge_count: self.charge_count,
            charge_reload_time: self.charge_reload_time,
        }
    }
}
