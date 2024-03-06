use crate::{ad, ec, shr::State};

pub(in crate::adg::custom) fn add_aar_paste_boost_effect(a_data: &mut ad::AData) {
    let effect = ad::AEffect::new(
        ec::effects::REE_AAR_PASTE_BOOST,
        State::Offline,
        None,
        false,
        false,
        false,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        ad::AModBuildStatus::Custom,
        // No modifiers, a custom one will be added later
        Vec::new(),
        Vec::new(),
        None,
    );
    a_data.effects.push(effect);
    for item in a_data.items.iter_mut().filter(|v| {
        v.effect_datas.contains_key(&ec::effects::FUELED_ARMOR_REPAIR)
            || v.effect_datas.contains_key(&ec::effects::SHIP_MODULE_ARAR)
    }) {
        item.effect_datas.insert(
            ec::effects::REE_AAR_PASTE_BOOST,
            ad::AItemEffData::new(None, None, None),
        );
    }
}
