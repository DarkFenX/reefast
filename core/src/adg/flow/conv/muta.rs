use itertools::Itertools;
use ordered_float::OrderedFloat as OF;

use crate::{ad, adg::EData, util::StMap};

pub(in crate::adg::flow::conv) fn conv_mutas(e_data: &EData) -> Vec<ad::AMuta> {
    let mut a_muta_map = StMap::new();
    for e_muta in e_data.muta_items.iter() {
        let a_muta = a_muta_map
            .entry(e_muta.muta_id)
            .or_insert_with(|| ad::AMuta::new(e_muta.muta_id));
        a_muta.item_map.insert(e_muta.in_item_id, e_muta.out_item_id);
    }
    for e_attr_data in e_data.muta_attrs.iter() {
        // We are interested in attribute modifiers only for mutators which have in-out item
        // definitions
        if let Some(a_muta) = a_muta_map.get_mut(&e_attr_data.muta_id) {
            a_muta.attr_mods.insert(
                e_attr_data.attr_id,
                ad::AMutaAttrRange {
                    min_mult: OF(e_attr_data.min_attr_mult),
                    max_mult: OF(e_attr_data.max_attr_mult),
                },
            );
        }
    }
    a_muta_map.into_iter().map(|(_, v)| v).sorted_by_key(|v| v.id).collect()
}
