use crate::{ac, ad};

pub(in crate::ntt::eff) fn update_effect(a_data: &mut ad::AData, effect_id: ad::AEffectId) {
    match a_data.effects.get_mut(&effect_id) {
        Some(effect) => {
            if !effect.mods.is_empty() {
                tracing::info!("web effect {effect_id} has modifiers, overwriting them");
                effect.mods.clear();
            }
            let modifier = ad::AEffectModifier {
                affector_attr_id: ac::attrs::SPEED_FACTOR,
                op: ad::AOp::PostPerc,
                affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Target),
                affectee_attr_id: ac::attrs::MAX_VELOCITY,
            };
            effect.mods.push(modifier);
            effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        }
        None => tracing::info!("web effect {effect_id} is not found for customization"),
    }
}
