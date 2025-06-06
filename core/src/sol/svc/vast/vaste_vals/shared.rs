use ordered_float::OrderedFloat as OF;

use crate::{
    ad,
    sol::{AttrVal, ItemKey, svc::calc::Calc, uad::Uad},
};

pub(super) fn get_max_resource(
    uad: &Uad,
    calc: &mut Calc,
    max_item_key: Option<ItemKey>,
    max_a_attr_id: &ad::AAttrId,
) -> Option<AttrVal> {
    calc.get_item_attr_val_extra_opt(uad, max_item_key, max_a_attr_id)
}

pub(super) fn is_flag_set(uad: &Uad, calc: &mut Calc, item_key: ItemKey, a_attr_id: &ad::AAttrId) -> bool {
    match calc.get_item_attr_val_extra(uad, item_key, a_attr_id) {
        Some(val) => val != OF(0.0),
        None => false,
    }
}
