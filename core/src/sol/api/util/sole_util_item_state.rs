use crate::{
    ad,
    def::ItemKey,
    sol::SolarSystem,
    svc::Svc,
    uad::{Uad, UadEffectUpdates, UadItem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_switch_item_state(
        uad: &Uad,
        svc: &mut Svc,
        item_key: ItemKey,
        old_item_a_state: ad::AState,
        new_item_a_state: ad::AState,
        eupdates: &UadEffectUpdates,
    ) {
        if new_item_a_state != old_item_a_state {
            let uad_item = uad.items.get(item_key);
            SolarSystem::util_internal_switch_item_state(
                uad,
                svc,
                item_key,
                uad_item,
                old_item_a_state,
                new_item_a_state,
                eupdates,
            );
        }
    }
    pub(in crate::sol::api::util) fn util_internal_switch_item_state(
        uad: &Uad,
        svc: &mut Svc,
        item_key: ItemKey,
        uad_item: &UadItem,
        old_item_a_state: ad::AState,
        new_item_a_state: ad::AState,
        eupdates: &UadEffectUpdates,
    ) {
        switch_item_state(svc, item_key, uad_item, old_item_a_state, new_item_a_state);
        SolarSystem::util_process_effect_updates(uad, svc, item_key, uad_item, eupdates);
    }
    pub(in crate::sol::api::util) fn util_internal_switch_item_state_without_projs(
        uad: &Uad,
        svc: &mut Svc,
        item_key: ItemKey,
        uad_item: &UadItem,
        old_item_a_state: ad::AState,
        new_item_a_state: ad::AState,
        eupdates: &UadEffectUpdates,
    ) {
        switch_item_state(svc, item_key, uad_item, old_item_a_state, new_item_a_state);
        SolarSystem::util_internal_process_effect_updates_without_projs(uad, svc, item_key, uad_item, eupdates);
    }
}

fn switch_item_state(
    svc: &mut Svc,
    item_key: ItemKey,
    uad_item: &UadItem,
    old_item_a_state: ad::AState,
    new_item_a_state: ad::AState,
) {
    match new_item_a_state.cmp(&old_item_a_state) {
        std::cmp::Ordering::Equal => (),
        std::cmp::Ordering::Greater => {
            let is_item_loaded = uad_item.is_loaded();
            for a_state in ad::AState::iter().filter(|v| **v > old_item_a_state && **v <= new_item_a_state) {
                svc.notify_state_activated(item_key, uad_item, a_state);
                if is_item_loaded {
                    svc.notify_item_state_activated_loaded(item_key, uad_item, a_state);
                }
            }
        }
        std::cmp::Ordering::Less => {
            let is_item_loaded = uad_item.is_loaded();
            for a_state in ad::AState::iter()
                .rev()
                .filter(|v| **v > new_item_a_state && **v <= old_item_a_state)
            {
                if is_item_loaded {
                    svc.notify_item_state_deactivated_loaded(&item_key, uad_item, a_state);
                }
                svc.notify_state_deactivated(&item_key, uad_item, a_state);
            }
        }
    }
}
