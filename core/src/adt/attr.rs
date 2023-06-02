use crate::{
    defs::{ReeFloat, ReeInt},
    util::Named,
};

/// Represents a dogma attribute.
///
/// An attribute carries just attribute properties which govern how modified attribute values are
/// calculated. Values themselves are stored on various items as plain numbers.
pub struct AAttr {
    /// Attribute ID.
    pub id: ReeInt,
    /// Defines if modifications applied to the attribute's values are immune to stacking penalties
    /// or not.
    pub penalizable: bool,
    /// "High is good" defines if higher value of the attribute is considered good or not.
    pub hig: bool,
    /// Default value of the attribute, used if not provided by an item type.
    pub def_val: Option<ReeFloat>,
    /// Refers another attribute, whose value limits value of this attribute.
    pub max_attr_id: Option<ReeInt>,
}
impl AAttr {
    /// Make a new dogma attribute out of passed data.
    pub(crate) fn new(
        id: ReeInt,
        penalizable: bool,
        hig: bool,
        def_val: Option<ReeFloat>,
        max_attr_id: Option<ReeInt>,
    ) -> Self {
        Self {
            id,
            penalizable,
            hig,
            def_val,
            max_attr_id,
        }
    }
}
impl Named for AAttr {
    fn get_name() -> &'static str {
        "ct::Attr"
    }
}
