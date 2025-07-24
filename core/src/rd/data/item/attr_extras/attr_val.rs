use crate::{ac, ad, def::OF, util::RMap};

pub(super) fn get_volume(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> ad::AAttrVal {
    match item_attrs.get(&ac::attrs::VOLUME) {
        Some(volume) => *volume,
        None => OF(0.0),
    }
}
pub(super) fn get_capacity(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> ad::AAttrVal {
    match item_attrs.get(&ac::attrs::CAPACITY) {
        Some(capacity) => *capacity,
        None => OF(0.0),
    }
}
pub(super) fn get_radius(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> ad::AAttrVal {
    match item_attrs.get(&ac::attrs::RADIUS) {
        Some(radius) => *radius,
        None => OF(0.0),
    }
}
pub(super) fn get_bandwidth_use(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> Option<ad::AAttrVal> {
    item_attrs.get(&ac::attrs::DRONE_BANDWIDTH_USED).copied()
}
pub(super) fn get_max_type_fitted_count(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> Option<ad::ACount> {
    item_attrs
        .get(&ac::attrs::MAX_TYPE_FITTED)
        .map(|v| v.round() as ad::ACount)
}
pub(super) fn get_online_max_sec_class(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> Option<ad::AAttrVal> {
    item_attrs.get(&ac::attrs::ONLINE_MAX_SECURITY_CLASS).copied()
}
pub(super) fn get_remote_resist_attr_id(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> Option<ad::AAttrId> {
    item_attrs
        .get(&ac::attrs::REMOTE_RESISTANCE_ID)
        .and_then(|val| match val {
            OF(0.0) => None,
            _ => Some(val.into_inner().round() as ad::AAttrId),
        })
}
pub(super) fn get_charge_size(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> Option<ad::AAttrVal> {
    item_attrs.get(&ac::attrs::CHARGE_SIZE).copied()
}
pub(super) fn get_charge_rate(item_attrs: &RMap<ad::AAttrId, ad::AAttrVal>) -> ad::ACount {
    match item_attrs.get(&ac::attrs::CHARGE_RATE) {
        Some(val) => val.round() as ad::ACount,
        None => 1,
    }
}
