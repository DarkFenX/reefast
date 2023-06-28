use std::collections::HashMap;

use crate::{
    defs::{AttrId, EffectCatId, EffectId},
    util::Named,
};

use super::EPrimitive;

/// EVE dogma effect data.
#[derive(Debug)]
pub struct EEffect {
    /// Effect ID.
    pub id: EffectId,
    /// Refers an effect category the effect belongs to.
    pub category_id: EffectCatId,
    /// Defines if the effect is considered as an assistance.
    pub is_assistance: bool,
    /// Defines if the effect is offensive or not.
    pub is_offensive: bool,
    /// Refers an attribute value which defines capacitor cost to run the effect.
    pub discharge_attr_id: Option<AttrId>,
    /// Refers an attribute value which defines how long an effect cycle would take in milliseconds.
    pub duration_attr_id: Option<AttrId>,
    /// Refers an attribute value which defines optimal range of the effect in meters.
    pub range_attr_id: Option<AttrId>,
    /// Refers an attribute value which defines falloff range of the effect in meters.
    pub falloff_attr_id: Option<AttrId>,
    /// Refers an attribute value which defines tracking speed of the effect.
    pub tracking_attr_id: Option<AttrId>,
    /// Refers an attribute value which defines chance of the effect to run when its parent item is
    /// fitted.
    pub usage_chance_attr_id: Option<AttrId>,
    /// Refers an attribute value which defines resistance strength to the effect.
    pub resist_attr_id: Option<AttrId>,
    /// Modifiers of the effect.
    pub mods: Vec<EEffectMod>,
}
impl EEffect {
    /// Make a new EVE dogma effect out of passed data.
    pub fn new(
        id: EffectId,
        category_id: EffectCatId,
        is_assistance: bool,
        is_offensive: bool,
        discharge_attr_id: Option<AttrId>,
        duration_attr_id: Option<AttrId>,
        range_attr_id: Option<AttrId>,
        falloff_attr_id: Option<AttrId>,
        tracking_attr_id: Option<AttrId>,
        usage_chance_attr_id: Option<AttrId>,
        resist_attr_id: Option<AttrId>,
        mods: Vec<EEffectMod>,
    ) -> Self {
        Self {
            id,
            category_id,
            is_assistance,
            is_offensive,
            discharge_attr_id,
            duration_attr_id,
            range_attr_id,
            falloff_attr_id,
            tracking_attr_id,
            usage_chance_attr_id,
            resist_attr_id,
            mods,
        }
    }
}
impl Named for EEffect {
    fn get_name() -> &'static str {
        "EEffect"
    }
}

/// EVE dogma effect modifier data.
#[derive(Debug)]
pub struct EEffectMod {
    /// Function which the effect modifier calls to apply its modification.
    pub func: String,
    /// Arguments to the function call.
    pub args: HashMap<String, EPrimitive>,
}
impl EEffectMod {
    /// Make a new EVE dogma effect modifier out of passed data.
    pub fn new(func: String, args: HashMap<String, EPrimitive>) -> Self {
        Self { func, args }
    }
}
