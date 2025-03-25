use smallvec::SmallVec;

use crate::{
    AttrVal, ac, ad,
    sol::{
        ItemId,
        svc::calc::{AffecteeFilter, AffectorInfo, AggrMode, Calc, Location, ModifierKind, Op},
        uad::{Uad, item::Item},
    },
};

use super::AffectorValue;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::calc) struct RawModifier {
    pub(in crate::sol::svc::calc) kind: ModifierKind,
    pub(in crate::sol::svc::calc) affector_item_id: ItemId,
    pub(in crate::sol::svc::calc) a_effect_id: ad::AEffectId,
    pub(in crate::sol::svc::calc::modifier) affector_value: AffectorValue,
    pub(in crate::sol::svc::calc) op: Op,
    pub(in crate::sol::svc::calc) aggr_mode: AggrMode,
    pub(in crate::sol::svc::calc) affectee_filter: AffecteeFilter,
    pub(in crate::sol::svc::calc) affectee_a_attr_id: ad::AAttrId,
    // Buff-related
    pub(in crate::sol::svc::calc) buff_type_a_attr_id: Option<ad::AAttrId>,
    // Projection-related
    pub(in crate::sol::svc::calc) resist_a_attr_id: Option<ad::AAttrId>,
    pub(in crate::sol::svc::calc) optimal_a_attr_id: Option<ad::AAttrId>,
    pub(in crate::sol::svc::calc) falloff_a_attr_id: Option<ad::AAttrId>,
}
impl RawModifier {
    pub(in crate::sol::svc::calc) fn from_a_modifier(
        affector_item: &Item,
        a_effect: &ad::AEffect,
        a_modifier: &ad::AEffectModifier,
    ) -> Option<Self> {
        let affectee_filter = AffecteeFilter::from_a_effect_affectee_filter(&a_modifier.affectee_filter, affector_item);
        let kind = get_mod_kind(a_effect, &affectee_filter)?;
        // Targeted effects are affected by both range and resists
        let (resist_a_attr_id, optimal_a_attr_id, falloff_a_attr_id) = match kind {
            ModifierKind::Targeted => (
                get_resist_a_attr_id(affector_item, a_effect),
                a_effect.range_attr_id,
                a_effect.falloff_attr_id,
            ),
            _ => (None, None, None),
        };
        Some(Self {
            kind,
            affector_item_id: affector_item.get_item_id(),
            a_effect_id: a_effect.id,
            affector_value: AffectorValue::AttrId(a_modifier.affector_attr_id),
            op: (&a_modifier.op).into(),
            aggr_mode: AggrMode::Stack,
            affectee_filter,
            affectee_a_attr_id: a_modifier.affectee_attr_id,
            buff_type_a_attr_id: None,
            resist_a_attr_id,
            optimal_a_attr_id,
            falloff_a_attr_id,
        })
    }
    pub(in crate::sol::svc::calc) fn from_a_buff_regular(
        affector_item: &Item,
        a_effect: &ad::AEffect,
        a_buff: &ad::ABuff,
        a_mod: &ad::ABuffModifier,
        affector_a_attr_id: ad::AAttrId,
        loc: Location,
        buff_type_a_attr_id: Option<ad::AAttrId>,
    ) -> Option<Self> {
        RawModifier::from_a_buff(
            affector_item,
            a_effect,
            a_buff,
            a_mod,
            AffectorValue::AttrId(affector_a_attr_id),
            loc,
            buff_type_a_attr_id,
        )
    }
    pub(in crate::sol::svc::calc) fn from_a_buff_hardcoded(
        affector_item: &Item,
        a_effect: &ad::AEffect,
        a_buff: &ad::ABuff,
        a_mod: &ad::ABuffModifier,
        affector_mod_val: AttrVal,
        loc: Location,
    ) -> Option<Self> {
        RawModifier::from_a_buff(
            affector_item,
            a_effect,
            a_buff,
            a_mod,
            AffectorValue::Hardcoded(affector_mod_val),
            loc,
            None,
        )
    }
    fn from_a_buff(
        affector_item: &Item,
        a_effect: &ad::AEffect,
        a_buff: &ad::ABuff,
        a_mod: &ad::ABuffModifier,
        affector_value: AffectorValue,
        loc: Location,
        buff_type_a_attr_id: Option<ad::AAttrId>,
    ) -> Option<Self> {
        let affectee_filter = AffecteeFilter::from_a_buff_affectee_filter(&a_mod.affectee_filter, loc, affector_item);
        let kind = get_mod_kind(a_effect, &affectee_filter)?;
        let (resist_a_attr_id, optimal_a_attr_id) = match kind {
            ModifierKind::Buff => (get_resist_a_attr_id(affector_item, a_effect), a_effect.range_attr_id),
            _ => (None, None),
        };
        Some(Self {
            kind,
            affector_item_id: affector_item.get_item_id(),
            a_effect_id: a_effect.id,
            affector_value,
            op: (&a_buff.op).into(),
            aggr_mode: AggrMode::from_a_buff(a_buff),
            affectee_filter,
            affectee_a_attr_id: a_mod.affectee_attr_id,
            buff_type_a_attr_id,
            resist_a_attr_id,
            optimal_a_attr_id,
            // Modifiers created from buffs never define falloff - buffs either apply fully, or they
            // don't
            falloff_a_attr_id: None,
        })
    }
    pub(in crate::sol::svc::calc) fn get_affector_a_attr_id(&self) -> Option<ad::AAttrId> {
        self.affector_value.get_affector_a_attr_id()
    }
    pub(in crate::sol::svc::calc) fn get_affector_info(&self, uad: &Uad) -> SmallVec<AffectorInfo, 1> {
        self.affector_value.get_affector_info(uad, &self.affector_item_id)
    }
    pub(in crate::sol::svc::calc) fn get_mod_val(&self, calc: &mut Calc, uad: &Uad) -> Option<AttrVal> {
        self.affector_value
            .get_mod_val(calc, uad, &self.affector_item_id, &self.a_effect_id)
    }
    // Revision methods - define if modification value can change upon some action
    pub(in crate::sol::svc::calc) fn needs_revision_on_item_add(&self) -> bool {
        self.affector_value.revisable_on_item_add()
    }
    pub(in crate::sol::svc::calc) fn needs_revision_on_item_remove(&self) -> bool {
        self.affector_value.revisable_on_item_remove()
    }
    pub(in crate::sol::svc::calc) fn revise_on_item_add(&self, added_item: &Item, uad: &Uad) -> bool {
        let affector_item = uad.items.get_item(&self.affector_item_id).unwrap();
        self.affector_value.revise_on_item_add(affector_item, added_item)
    }
    pub(in crate::sol::svc::calc) fn revise_on_item_remove(&self, added_item: &Item, uad: &Uad) -> bool {
        let affector_item = uad.items.get_item(&self.affector_item_id).unwrap();
        self.affector_value.revise_on_item_remove(affector_item, added_item)
    }
}

fn get_mod_kind(a_effect: &ad::AEffect, a_affectee_filter: &AffecteeFilter) -> Option<ModifierKind> {
    if let AffecteeFilter::Direct(loc) = a_affectee_filter {
        if matches!(loc, Location::Item | Location::Other) {
            return Some(ModifierKind::Local);
        }
    }
    match (a_effect.category, &a_effect.buff) {
        // Local modifications
        (ac::effcats::PASSIVE | ac::effcats::ACTIVE | ac::effcats::ONLINE | ac::effcats::OVERLOAD, None) => {
            Some(ModifierKind::Local)
        }
        // Buffs
        (ac::effcats::ACTIVE, Some(a_buff_info)) => match a_buff_info.scope {
            ad::AEffectBuffScope::FleetShips => Some(ModifierKind::FleetBuff),
            _ => Some(ModifierKind::Buff),
        },
        // Lib system-wide effects are EVE system effects and buffs
        (ac::effcats::SYSTEM, None) => Some(ModifierKind::System),
        // Targeted effects
        (ac::effcats::TARGET, None) => Some(ModifierKind::Targeted),
        _ => None,
    }
}

pub(in crate::sol::svc::calc) fn get_resist_a_attr_id(item: &Item, a_effect: &ad::AEffect) -> Option<ad::AAttrId> {
    match a_effect.resist_attr_id {
        Some(resist_a_attr_id) => Some(resist_a_attr_id),
        None => match item.get_a_attrs() {
            Some(a_attrs) => match a_attrs
                .get(&ac::attrs::REMOTE_RESISTANCE_ID)
                .map(|v| v.into_inner() as ad::AAttrId)
            {
                Some(a_attr_id) if a_attr_id != 0 => Some(a_attr_id),
                _ => None,
            },
            None => None,
        },
    }
}
