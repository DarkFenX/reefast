use ordered_float::OrderedFloat as OF;

use crate::{
    ad,
    sol::{AttrVal, Count, ItemId, svc::calc::Calc, uad::Uad},
};

pub(super) fn get_max_resource(
    uad: &Uad,
    calc: &mut Calc,
    max_item_id: &Option<ItemId>,
    max_a_attr_id: &ad::AAttrId,
) -> Option<AttrVal> {
    calc.get_item_attr_val_extra_opt(uad, max_item_id, max_a_attr_id)
}

pub(super) fn get_max_slots(
    uad: &Uad,
    calc: &mut Calc,
    max_item_id: &Option<ItemId>,
    max_a_attr_id: &ad::AAttrId,
) -> Option<Count> {
    calc.get_item_attr_val_extra_opt(uad, max_item_id, max_a_attr_id)
        .map(|v| v.into_inner().round() as Count)
}

pub(super) fn is_flag_set(uad: &Uad, calc: &mut Calc, item_id: &ItemId, a_attr_id: &ad::AAttrId) -> bool {
    match calc.get_item_attr_val_extra(uad, item_id, a_attr_id) {
        Some(val) => val != OF(0.0),
        None => match uad
            .items
            .get_item(item_id)
            .unwrap()
            .get_a_attrs()
            .unwrap()
            .get(a_attr_id)
        {
            Some(a_val) => *a_val != OF(0.0),
            None => false,
        },
    }
}
