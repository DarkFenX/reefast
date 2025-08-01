use super::is_a_effect_projectable;
use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItem, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_process_effect_updates(
        u_data: &UData,
        svc: &mut Svc,
        item_key: UItemKey,
        u_item: &UItem,
        eupdates: &UEffectUpdates,
    ) {
        process_effect_updates(u_data, svc, item_key, u_item, eupdates, true);
    }
    pub(in crate::sol::api::util) fn util_internal_process_effect_updates_without_projs(
        u_data: &UData,
        svc: &mut Svc,
        item_key: UItemKey,
        u_item: &UItem,
        eupdates: &UEffectUpdates,
    ) {
        process_effect_updates(u_data, svc, item_key, u_item, eupdates, false);
    }
}

fn process_effect_updates(
    u_data: &UData,
    svc: &mut Svc,
    item_key: UItemKey,
    u_item: &UItem,
    eupdates: &UEffectUpdates,
    handle_projs: bool,
) {
    if !eupdates.to_start.is_empty() {
        svc.notify_effects_started(u_data, item_key, u_item, &eupdates.to_start);
        if handle_projs && let Some(projs) = u_item.iter_projs() {
            for (projectee_key, range) in projs {
                let projectee_item = u_data.items.get(projectee_key);
                for r_effect in eupdates.to_start.iter() {
                    if is_a_effect_projectable(u_item, r_effect) {
                        svc.notify_effect_projected(
                            u_data,
                            item_key,
                            u_item,
                            r_effect,
                            projectee_key,
                            projectee_item,
                            range,
                        );
                    }
                }
            }
        }
    }
    if !eupdates.to_stop.is_empty() {
        if handle_projs && let Some(projectee_keys) = u_item.iter_projectees() {
            for projectee_key in projectee_keys {
                let projectee_item = u_data.items.get(projectee_key);
                for r_effect in eupdates.to_stop.iter() {
                    if is_a_effect_projectable(u_item, r_effect) {
                        svc.notify_effect_unprojected(
                            u_data,
                            item_key,
                            u_item,
                            r_effect,
                            projectee_key,
                            projectee_item,
                        );
                    }
                }
            }
        }
        svc.notify_effects_stopped(u_data, item_key, u_item, &eupdates.to_stop);
    }
}
