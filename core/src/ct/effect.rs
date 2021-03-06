use crate::{
    consts::{ModAfeeFilter, ModAggrMode, ModOp, State, TgtMode},
    defines::ReeInt,
};

/// Represents a dogma effect.
///
/// Effects are higher-level modification descriptors, as opposed to modifiers, which are
/// lower-level. An effect can contain any amount of modifiers under a single roof, accompanied by
/// extra effect-wide properties.
#[derive(Debug)]
pub struct Effect {
    /// Effect ID.
    pub id: ReeInt,
    /// Effect state dictates which state of parent item is needed for the effect to run.
    pub state: State,
    /// Defines what kind of target you need to run the effect.
    pub tgt_mode: TgtMode,
    /// Defines if the effect is considered as an assistance.
    pub is_assist: bool,
    /// Defines if the effect is offensive or not.
    pub is_offense: bool,
    /// Defines if the effect can be used in hisec.
    pub hisec: Option<bool>,
    /// Defines if the effect can be used in lowsec.
    pub lowsec: Option<bool>,
    /// Refers an attribute value which defines capacitor cost to run the effect.
    pub discharge_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines how long an effect cycle would take in milliseconds.
    pub duration_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines optimal range of the effect in meters.
    pub range_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines falloff range of the effect in meters.
    pub falloff_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines tracking speed of the effect.
    pub track_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines chance of the effect to run when its parent item is
    /// fitted.
    pub chance_attr_id: Option<ReeInt>,
    /// Refers an attribute value which defines resistance strength to the effect.
    pub resist_attr_id: Option<ReeInt>,
    /// Attribute modifiers carried by the effect
    pub mods: Vec<AttrMod>,
    /// Refers effects this effect stops.
    pub stop_ids: Vec<ReeInt>,
}
impl Effect {
    /// Make a new dogma effect out of passed data.
    pub fn new(
        id: ReeInt,
        state: State,
        tgt_mode: TgtMode,
        is_assist: bool,
        is_offense: bool,
        hisec: Option<bool>,
        lowsec: Option<bool>,
        discharge_attr_id: Option<ReeInt>,
        duration_attr_id: Option<ReeInt>,
        range_attr_id: Option<ReeInt>,
        falloff_attr_id: Option<ReeInt>,
        track_attr_id: Option<ReeInt>,
        chance_attr_id: Option<ReeInt>,
        resist_attr_id: Option<ReeInt>,
        mods: Vec<AttrMod>,
        stop_ids: Vec<ReeInt>,
    ) -> Effect {
        Effect {
            id,
            state,
            tgt_mode,
            is_assist,
            is_offense,
            hisec,
            lowsec,
            discharge_attr_id,
            duration_attr_id,
            range_attr_id,
            falloff_attr_id,
            track_attr_id,
            chance_attr_id,
            resist_attr_id,
            mods,
            stop_ids,
        }
    }
}

/// An attribute modifier.
///
/// A modifier is an entity which specifies in detail which attributes on which items are going to
/// be affected, and how.
#[derive(Debug)]
pub struct AttrMod {
    /// Refers an attribute on the affector, which should be used as modification value.
    pub afor_attr_id: ReeInt,
    /// Defines how multiple modifications of the same attribute value are aggregated.
    pub aggr_mode: ModAggrMode,
    /// Operation to apply during the modification.
    pub op: ModOp,
    /// Defines an affectee filter, that is a filter which defines which items will be affected.
    pub afee_filter: ModAfeeFilter,
    /// Refers an attribute, whose value will be affected on the affectee.
    pub afee_attr_id: ReeInt,
}
impl AttrMod {
    /// Make a new attribute modifier out of passed data.
    pub fn new(
        afor_attr_id: ReeInt,
        aggr_mode: ModAggrMode,
        op: ModOp,
        afee_filter: ModAfeeFilter,
        afee_attr_id: ReeInt,
    ) -> AttrMod {
        AttrMod {
            afor_attr_id,
            aggr_mode,
            op,
            afee_filter,
            afee_attr_id,
        }
    }
}
