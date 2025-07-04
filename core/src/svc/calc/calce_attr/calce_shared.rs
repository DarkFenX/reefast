use ordered_float::Float;

use crate::{
    ac, ad,
    def::{AttrVal, OF},
    misc::AttrSpec,
    svc::{
        SvcCtx,
        calc::{Calc, Context, CtxModifier, ModifierKind},
        get_resist_mult_val_by_projectee_aspec,
    },
    uad::UadItem,
};

pub(super) const LIMITED_PRECISION_A_ATTR_IDS: [ad::AAttrId; 4] = [
    ac::attrs::CPU,
    ac::attrs::POWER,
    ac::attrs::CPU_OUTPUT,
    ac::attrs::POWER_OUTPUT,
];

impl Calc {
    pub(super) fn calc_resist_mult(&mut self, ctx: SvcCtx, cmod: &CtxModifier) -> Option<AttrVal> {
        // Only buffs and targeted modifiers can be resisted
        if !matches!(cmod.raw.kind, ModifierKind::Buff | ModifierKind::Targeted) {
            return None;
        }
        let resist_a_attr_id = cmod.raw.resist_a_attr_id?;
        let projectee_key = match cmod.ctx {
            Context::Item(projectee_key) => projectee_key,
            _ => return None,
        };
        let resist =
            get_resist_mult_val_by_projectee_aspec(ctx, self, &AttrSpec::new(projectee_key, resist_a_attr_id))?;
        Some(resist)
    }
    pub(super) fn calc_proj_mult(&mut self, ctx: SvcCtx, cmod: &CtxModifier) -> Option<AttrVal> {
        let projectee_key = match cmod.ctx {
            Context::Item(projectee_key) => projectee_key,
            _ => return None,
        };
        let proj_range = ctx.eprojs.get_range(cmod.raw.affector_espec, projectee_key)?;
        match cmod.raw.kind {
            ModifierKind::Targeted => self.calc_proj_mult_targeted(ctx, cmod, proj_range.s2s),
            ModifierKind::Buff => self.calc_proj_mult_buff(ctx, cmod, proj_range.s2s),
            _ => None,
        }
    }
    // Private methods
    fn calc_proj_mult_targeted(&mut self, ctx: SvcCtx, cmod: &CtxModifier, proj_range: AttrVal) -> Option<AttrVal> {
        // Assume optimal range is 0 if it's not available
        let affector_optimal = match cmod.raw.optimal_a_attr_id {
            Some(optimal_a_attr_id) => {
                match self.get_item_attr_val_full(ctx, cmod.raw.affector_espec.item_key, &optimal_a_attr_id) {
                    Ok(val) => val.dogma,
                    _ => OF(0.0),
                }
            }
            None => OF(0.0),
        };
        // Assume falloff range is 0 if it's not available
        let affector_falloff = match cmod.raw.falloff_a_attr_id {
            Some(falloff_a_attr_id) => {
                match self.get_item_attr_val_full(ctx, cmod.raw.affector_espec.item_key, &falloff_a_attr_id) {
                    Ok(val) => val.dogma,
                    _ => OF(0.0),
                }
            }
            None => OF(0.0),
        };
        // TODO: do not hardcode it here, define on a per-effect basis
        let restricted_range = false;
        // Calculate actual range multiplier after collecting all the data
        if affector_falloff > OF(0.0) {
            if restricted_range && proj_range > affector_optimal + OF(3.0) * affector_falloff {
                Some(OF(0.0))
            } else {
                let val = Float::powf(
                    OF(0.5),
                    (Float::max(OF(0.0), proj_range - affector_optimal) / affector_falloff).powi(2),
                );
                Some(val)
            }
        } else if proj_range <= affector_optimal {
            Some(OF(1.0))
        } else {
            Some(OF(0.0))
        }
    }
    fn calc_proj_mult_buff(&mut self, ctx: SvcCtx, cmod: &CtxModifier, proj_range: AttrVal) -> Option<AttrVal> {
        let affector_optimal = match cmod.raw.optimal_a_attr_id {
            Some(optimal_a_attr_id) => {
                match self.get_item_attr_val_full(ctx, cmod.raw.affector_espec.item_key, &optimal_a_attr_id) {
                    Ok(val) => val.dogma,
                    // If optimal range attribute ID is defined but value is not available, assume
                    // optimal range of 0
                    _ => OF(0.0),
                }
            }
            // If optimal range attribute ID not defined, assume buff is not restricted by range
            None => return None,
        };
        if proj_range <= affector_optimal {
            Some(OF(1.0))
        } else {
            Some(OF(0.0))
        }
    }
}

pub(super) fn get_a_attr(a_attr_id: ad::AAttrId) -> ad::AAttr {
    ad::AAttr {
        id: a_attr_id,
        penalizable: false,
        hig: true,
        def_val: OF(0.0),
        min_attr_id: None,
        max_attr_id: None,
    }
}

pub(super) fn get_base_attr_value(item: &UadItem, a_attr: &ad::AAttr) -> AttrVal {
    // Fetch unmodified on-item attribute value, or use base attribute value if it is not available
    match item.get_a_attrs().unwrap().get(&a_attr.id) {
        Some(orig_val) => *orig_val as AttrVal,
        None => a_attr.def_val as AttrVal,
    }
}
