use crate::{
    ac,
    ad::{AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AOp},
};

pub(in crate::nd::eff) fn update_effect(a_effect_id: AEffectId, a_effect: &mut AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {a_effect_id}: TP effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    a_effect.mods.push(AEffectModifier {
        affector_attr_id: ac::attrs::SIG_RADIUS_BONUS,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
        affectee_attr_id: ac::attrs::SIG_RADIUS,
    });
}
