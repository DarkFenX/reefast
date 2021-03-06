use std::collections::HashMap;

use crate::{
    consts::ItemType,
    defines::{ReeFloat, ReeInt},
};

/// Represents an item.
///
/// An item carries alot of info needed to calculate fit attributes, for example base attribute
/// values.
#[derive(Debug)]
pub struct Item {
    /// Item ID.
    pub id: ReeInt,
    /// Item type.
    pub itype: ItemType,
    /// Item group ID.
    pub grp_id: ReeInt,
    /// Item category ID.
    pub cat_id: ReeInt,
    /// Attribute values of the item.
    pub attr_vals: HashMap<ReeInt, ReeFloat>,
    /// Refers effects of the item.
    pub effect_datas: HashMap<ReeInt, ItemEffData>,
    /// Refers an effect which is default for the item.
    pub defeff_id: Option<ReeInt>,
    /// Skill requirement map.
    pub srqs: HashMap<ReeInt, ReeInt>,
}
impl Item {
    /// Make a new item out of passed data.
    pub fn new(
        id: ReeInt,
        itype: ItemType,
        grp_id: ReeInt,
        cat_id: ReeInt,
        attr_vals: HashMap<ReeInt, ReeFloat>,
        effect_datas: HashMap<ReeInt, ItemEffData>,
        defeff_id: Option<ReeInt>,
        srqs: HashMap<ReeInt, ReeInt>,
    ) -> Item {
        Item {
            id,
            itype,
            grp_id,
            cat_id,
            attr_vals,
            effect_datas,
            defeff_id,
            srqs,
        }
    }
}

/// Stores item-specific effect data.
#[derive(Debug)]
pub struct ItemEffData {
    /// Defines cooldown of the effect in seconds.
    pub cd: Option<ReeFloat>,
    /// Defines how many times the effect can be used before its parent item has to reload.
    pub charges: Option<ReeInt>,
    /// Defines how much time each charge of the effect takes to reload, in seconds.
    pub charge_reload_time: Option<ReeFloat>,
}
impl ItemEffData {
    /// Make a new per-item effect data container out of passed data.
    pub fn new(cd: Option<ReeFloat>, charges: Option<ReeInt>, charge_reload_time: Option<ReeFloat>) -> ItemEffData {
        ItemEffData {
            cd,
            charges,
            charge_reload_time,
        }
    }
}
