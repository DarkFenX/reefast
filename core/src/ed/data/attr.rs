use crate::{
    ed::{EAttrId, EAttrUnitId, EAttrVal},
    util::Named,
};

/// EVE dogma attribute data.
pub struct EAttr {
    /// Attribute ID.
    pub id: EAttrId,
    /// Defines if modifications applied to the attribute's values stack with penalty (false) or not
    /// (true).
    pub stackable: bool,
    /// Defines if higher value of the attribute is considered good or not.
    pub high_is_good: bool,
    /// Default value of the attribute, used if not provided by an item type.
    pub default_value: EAttrVal,
    /// Refers another attribute, whose value limits minimum value of this attribute.
    pub min_attr_id: Option<EAttrId>,
    /// Refers another attribute, whose value limits maximum value of this attribute.
    pub max_attr_id: Option<EAttrId>,
    /// Defines what kind of unit is used for the attribute's value. Used during cache generation
    /// process during cleanup, since this field defines if value of the attribute refers another
    /// attribute, group or something else.
    pub unit_id: Option<EAttrUnitId>,
}
impl Named for EAttr {
    fn get_name() -> &'static str {
        "EAttr"
    }
}
