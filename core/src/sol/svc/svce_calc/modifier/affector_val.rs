use num_traits::cast::ToPrimitive;

use crate::{
    defs::{AttrVal, EAttrId, EEffectId, Rational, SolItemId},
    sol::{item::SolItem, svc::SolSvcs, SolView},
    util::Result,
};

use super::custom::{aar, prop};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) enum SolAffectorValue {
    AttrId(EAttrId),
    Hardcoded(Rational),
    PropulsionModule,
    AncillaryArmorRep,
}
impl SolAffectorValue {
    // Simple and fast way to get affector attribute
    pub(super) fn get_affector_attr_id(&self) -> Option<EAttrId> {
        match self {
            Self::AttrId(attr_id) => Some(*attr_id),
            Self::Hardcoded(_) => None,
            Self::PropulsionModule => None,
            Self::AncillaryArmorRep => Some(aar::AAR_AFFECTOR_ATTR_ID),
        }
    }
    // More expensive, but comprehensive info about affecting items/attributes
    pub(super) fn get_affectors(&self, sol_view: &SolView, item_id: &SolItemId) -> Vec<(SolItemId, EAttrId)> {
        match self {
            Self::AttrId(attr_id) => vec![(*item_id, *attr_id)],
            Self::Hardcoded(_) => Vec::new(),
            Self::PropulsionModule => prop::get_affectors(sol_view, item_id),
            Self::AncillaryArmorRep => vec![(*item_id, aar::AAR_AFFECTOR_ATTR_ID)],
        }
    }
    pub(super) fn get_mod_val(
        &self,
        svc: &mut SolSvcs,
        sol_view: &SolView,
        item_id: &SolItemId,
        effect_id: &EEffectId,
    ) -> Result<AttrVal> {
        match self {
            Self::AttrId(attr_id) => Ok(svc.calc_get_item_attr_val(sol_view, item_id, attr_id)?.dogma),
            Self::Hardcoded(val_rational) => Ok(val_rational.to_f64().unwrap()),
            Self::PropulsionModule => prop::get_mod_val(svc, sol_view, item_id, effect_id),
            Self::AncillaryArmorRep => aar::get_mod_val(svc, sol_view, item_id),
        }
    }
    // Revision methods - define if modification value can change upon some action
    pub(super) fn revisable_on_item_add(&self) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::PropulsionModule => false,
            Self::AncillaryArmorRep => true,
        }
    }
    pub(super) fn revisable_on_item_remove(&self) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::PropulsionModule => false,
            Self::AncillaryArmorRep => true,
        }
    }
    pub(super) fn revise_on_item_add(&self, affector_item: &SolItem, changed_item: &SolItem) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::PropulsionModule => false,
            Self::AncillaryArmorRep => aar::revise_on_item_add_removal(affector_item, changed_item),
        }
    }
    pub(super) fn revise_on_item_remove(&self, affector_item: &SolItem, changed_item: &SolItem) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::PropulsionModule => false,
            Self::AncillaryArmorRep => aar::revise_on_item_add_removal(affector_item, changed_item),
        }
    }
}
