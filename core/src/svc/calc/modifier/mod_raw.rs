use std::hash::{Hash, Hasher};

use smallvec::SmallVec;

use super::AffectorValue;
use crate::{
    ac, ad,
    def::AttrVal,
    misc::EffectSpec,
    nd::NProjMultGetter,
    rd,
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, AffectorInfo, AggrMode, Calc, ItemAddReviser, ItemRemoveReviser, Location, ModifierKind, Op,
        },
        efuncs,
    },
    ud::{UItem, UItemKey},
};

#[derive(Copy, Clone)]
pub(crate) struct RawModifier {
    pub(crate) kind: ModifierKind,
    pub(crate) affector_espec: EffectSpec,
    pub(crate) affector_value: AffectorValue,
    pub(crate) op: Op,
    pub(crate) aggr_mode: AggrMode,
    pub(crate) affectee_filter: AffecteeFilter,
    pub(crate) affectee_a_attr_id: ad::AAttrId,
    // Buff-related
    pub(crate) buff_type_a_attr_id: Option<ad::AAttrId> = None,
    // Projection-related
    pub(crate) resist_a_attr_id: Option<ad::AAttrId> = None,
    pub(crate) proj_mult_getter: Option<NProjMultGetter> = None,
    pub(crate) proj_a_attr_ids: [Option<ad::AAttrId>; 2] = [None, None],
}
impl PartialEq for RawModifier {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
            && self.affector_espec.eq(&other.affector_espec)
            && self.affector_value.eq(&other.affector_value)
            && self.op.eq(&other.op)
            && self.aggr_mode.eq(&other.aggr_mode)
            && self.affectee_filter.eq(&other.affectee_filter)
            && self.affectee_a_attr_id.eq(&other.affectee_a_attr_id)
            && self.buff_type_a_attr_id.eq(&other.buff_type_a_attr_id)
    }
}
impl Eq for RawModifier {}
impl Hash for RawModifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.affector_espec.hash(state);
        self.affector_value.hash(state);
        self.op.hash(state);
        self.aggr_mode.hash(state);
        self.affectee_filter.hash(state);
        self.affectee_a_attr_id.hash(state);
        self.buff_type_a_attr_id.hash(state);
    }
}
impl RawModifier {
    pub(in crate::svc::calc) fn try_from_amod(
        affector_key: UItemKey,
        affector_item: &UItem,
        r_effect: &rd::REffect,
        amod: &ad::AEffectModifier,
    ) -> Option<Self> {
        let affectee_filter = AffecteeFilter::from_a_affectee_filter(&amod.affectee_filter, affector_item);
        let kind = get_mod_kind(r_effect, &affectee_filter)?;
        // Targeted effects are affected resists
        let resist_a_attr_id = match kind {
            ModifierKind::Targeted => efuncs::get_resist_a_attr_id(affector_item, r_effect),
            _ => None,
        };
        Some(Self {
            kind,
            affector_espec: EffectSpec::new(affector_key, r_effect.get_id()),
            affector_value: AffectorValue::AttrId(amod.affector_attr_id),
            op: (&amod.op).into(),
            aggr_mode: AggrMode::Stack,
            affectee_filter,
            affectee_a_attr_id: amod.affectee_attr_id,
            buff_type_a_attr_id: None,
            resist_a_attr_id,
            proj_mult_getter: r_effect.get_proj_mult_getter(),
            proj_a_attr_ids: r_effect.get_proj_a_attr_ids(),
            ..
        })
    }
    pub(in crate::svc::calc) fn try_from_r_buff_regular(
        affector_key: UItemKey,
        affector_item: &UItem,
        r_effect: &rd::REffect,
        r_buff: &rd::RBuff,
        a_mod: &ad::ABuffModifier,
        affector_a_attr_id: ad::AAttrId,
        loc: Location,
        buff_type_a_attr_id: Option<ad::AAttrId>,
    ) -> Option<Self> {
        RawModifier::from_r_buff(
            affector_key,
            affector_item,
            r_effect,
            r_buff,
            a_mod,
            AffectorValue::AttrId(affector_a_attr_id),
            loc,
            buff_type_a_attr_id,
        )
    }
    pub(in crate::svc::calc) fn try_from_r_buff_hardcoded(
        affector_key: UItemKey,
        affector_item: &UItem,
        r_effect: &rd::REffect,
        r_buff: &rd::RBuff,
        a_mod: &ad::ABuffModifier,
        affector_mod_val: AttrVal,
        loc: Location,
    ) -> Option<Self> {
        RawModifier::from_r_buff(
            affector_key,
            affector_item,
            r_effect,
            r_buff,
            a_mod,
            AffectorValue::Hardcoded(affector_mod_val),
            loc,
            None,
        )
    }
    fn from_r_buff(
        affector_key: UItemKey,
        affector_item: &UItem,
        r_effect: &rd::REffect,
        r_buff: &rd::RBuff,
        a_mod: &ad::ABuffModifier,
        affector_value: AffectorValue,
        loc: Location,
        buff_type_a_attr_id: Option<ad::AAttrId>,
    ) -> Option<Self> {
        let affectee_filter = AffecteeFilter::from_a_buff_affectee_filter(&a_mod.affectee_filter, loc, affector_item);
        let kind = get_mod_kind(r_effect, &affectee_filter)?;
        let resist_a_attr_id = match kind {
            ModifierKind::Buff => efuncs::get_resist_a_attr_id(affector_item, r_effect),
            _ => None,
        };
        Some(Self {
            kind,
            affector_espec: EffectSpec::new(affector_key, r_effect.get_id()),
            affector_value,
            op: (&r_buff.get_op()).into(),
            aggr_mode: AggrMode::from_r_buff(r_buff),
            affectee_filter,
            affectee_a_attr_id: a_mod.affectee_attr_id,
            buff_type_a_attr_id,
            resist_a_attr_id,
            proj_mult_getter: r_effect.get_proj_mult_getter(),
            proj_a_attr_ids: r_effect.get_proj_a_attr_ids(),
            ..
        })
    }
    pub(in crate::svc::calc) fn get_affector_a_attr_id(&self) -> Option<ad::AAttrId> {
        self.affector_value.get_affector_a_attr_id()
    }
    pub(in crate::svc::calc) fn get_affector_info(&self, ctx: SvcCtx) -> SmallVec<AffectorInfo, 1> {
        self.affector_value.get_affector_info(ctx, self.affector_espec.item_key)
    }
    pub(in crate::svc::calc) fn get_mod_val(&self, calc: &mut Calc, ctx: SvcCtx) -> Option<AttrVal> {
        self.affector_value.get_mod_val(calc, ctx, self.affector_espec)
    }
    // Revision methods - define if modification value can change upon some action
    pub(in crate::svc::calc) fn get_item_add_reviser(&self) -> Option<ItemAddReviser> {
        self.affector_value.get_item_add_reviser()
    }
    pub(in crate::svc::calc) fn get_item_remove_reviser(&self) -> Option<ItemRemoveReviser> {
        self.affector_value.get_item_remove_reviser()
    }
}

fn get_mod_kind(r_effect: &rd::REffect, affectee_filter: &AffecteeFilter) -> Option<ModifierKind> {
    if let AffecteeFilter::Direct(loc) = affectee_filter
        && matches!(loc, Location::Item | Location::Other)
    {
        return Some(ModifierKind::Local);
    }
    match (r_effect.get_category(), &r_effect.get_a_buff_info()) {
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
