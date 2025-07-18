use crate::{
    ac, ad,
    misc::EffectMode,
    src::Src,
    uad::item::misc::EffectModes,
    util::{RMap, RSet},
};

const ONLINE_EFFECT_ID: ad::AEffectId = ac::effects::ONLINE;

pub(crate) struct UadEffectUpdates {
    pub(crate) to_start: Vec<ad::ArcEffectRt>,
    pub(crate) to_stop: Vec<ad::ArcEffectRt>,
}
impl UadEffectUpdates {
    pub(crate) fn new() -> Self {
        Self {
            to_start: Vec::new(),
            to_stop: Vec::new(),
        }
    }
    pub(in crate::uad::item) fn clear(&mut self) {
        self.to_start.clear();
        self.to_stop.clear();
    }
}

pub(super) fn process_effects(
    reuse_eupdates: &mut UadEffectUpdates,
    reffs: &mut RSet<ad::AEffectId>,
    src: &Src,
    item_effect_datas: &RMap<ad::AEffectId, ad::AItemEffectData>,
    item_defeff_id: Option<ad::AEffectId>,
    item_a_state: ad::AState,
    item_effect_modes: &EffectModes,
    item_xt: &ad::AItemXt,
) {
    match item_a_state {
        ad::AState::Ghost => stop_all_effects(reuse_eupdates, reffs, src),
        _ => update_running_effects(
            reuse_eupdates,
            reffs,
            src,
            item_effect_datas,
            item_defeff_id,
            item_a_state,
            item_effect_modes,
            item_xt,
        ),
    }
}

fn stop_all_effects(reuse_eupdates: &mut UadEffectUpdates, reffs: &mut RSet<ad::AEffectId>, src: &Src) {
    // We don't want waste time resolving effects when we want them to just stop (which happens
    // before e.g. item removal)
    reuse_eupdates.to_stop.extend(
        reffs
            .drain()
            .map(|a_effect_id| src.get_a_effect(&a_effect_id).unwrap().clone()),
    );
}

fn update_running_effects(
    reuse_eupdates: &mut UadEffectUpdates,
    reffs: &mut RSet<ad::AEffectId>,
    src: &Src,
    item_effect_datas: &RMap<ad::AEffectId, ad::AItemEffectData>,
    item_defeff_id: Option<ad::AEffectId>,
    item_a_state: ad::AState,
    item_effect_modes: &EffectModes,
    item_xt: &ad::AItemXt,
) {
    // Separate handling for the online effect
    let online_should_run = resolve_online_effect_status(item_xt, item_effect_modes, item_a_state);
    let online_running = reffs.contains(&ONLINE_EFFECT_ID);
    // Whenever online effect status changes, it should be guaranteed that online effect is
    // available on the source level, so can just unwrap here
    if online_running && !online_should_run {
        reuse_eupdates.to_stop.push(src.get_a_effect_online().unwrap().clone());
    } else if !online_running && online_should_run {
        reuse_eupdates.to_start.push(src.get_a_effect_online().unwrap().clone());
    }
    for &a_effect_id in item_effect_datas.keys() {
        // Online effect has already been handled
        if a_effect_id == ONLINE_EFFECT_ID {
            continue;
        }
        let a_effect = match src.get_a_effect(&a_effect_id) {
            Some(a_effect) => a_effect,
            None => continue,
        };
        let should_run = resolve_regular_effect_status(
            item_effect_modes,
            item_defeff_id,
            item_a_state,
            online_should_run,
            a_effect,
        );
        let running = reffs.contains(&a_effect.ae.id);
        if running && !should_run {
            reuse_eupdates.to_stop.push(a_effect.clone());
        } else if !running && should_run {
            reuse_eupdates.to_start.push(a_effect.clone());
        };
    }
    reffs.extend(reuse_eupdates.to_start.iter().map(|a_effect| a_effect.ae.id));
    for a_effect in reuse_eupdates.to_stop.iter() {
        reffs.remove(&a_effect.ae.id);
    }
}

fn resolve_online_effect_status(
    item_xt: &ad::AItemXt,
    item_effect_modes: &EffectModes,
    item_a_state: ad::AState,
) -> bool {
    if !item_xt.has_online_effect {
        return false;
    }
    match item_effect_modes.get(&ONLINE_EFFECT_ID) {
        // Since other effects from online category depend on the online effect in full compliance
        // mode, use simplified resolution for the online effect itself
        EffectMode::FullCompliance | EffectMode::StateCompliance => item_a_state >= ad::AState::Online,
        // Shouldn't run anything in ghost state even with force run mode
        EffectMode::ForceRun => item_a_state != ad::AState::Ghost,
        EffectMode::ForceStop => false,
    }
}

fn resolve_regular_effect_status(
    item_effect_modes: &EffectModes,
    item_defeff_id: Option<ad::AEffectId>,
    item_a_state: ad::AState,
    online_running: bool,
    a_effect: &ad::AEffectRt,
) -> bool {
    // Ghosted items should never affect anything regardless of effect mode, so check it first
    // wherever applicable
    match item_effect_modes.get(&a_effect.ae.id) {
        EffectMode::FullCompliance => {
            item_a_state != ad::AState::Ghost
                && resolve_regular_effect_status_full(item_defeff_id, item_a_state, a_effect, online_running)
        }
        EffectMode::StateCompliance => item_a_state != ad::AState::Ghost && item_a_state >= a_effect.ae.state,
        EffectMode::ForceRun => item_a_state != ad::AState::Ghost,
        EffectMode::ForceStop => false,
    }
}

fn resolve_regular_effect_status_full(
    item_defeff_id: Option<ad::AEffectId>,
    item_a_state: ad::AState,
    a_effect: &ad::AEffectRt,
    online_running: bool,
) -> bool {
    match a_effect.ae.state {
        ad::AState::Ghost => unreachable!("ghost state should never reach full resolver"),
        // Offline effects require item in offline+ state, and no fitting usage chance attribute
        // (not to run booster side effects by default)
        ad::AState::Offline => item_a_state >= a_effect.ae.state && a_effect.ae.chance_attr_id.is_none(),
        // Online effects depend on 'online' effect, ignoring everything else
        ad::AState::Online => online_running,
        // Only default active effect is run, and only if item is in active+ state
        ad::AState::Active => {
            if a_effect.ae.state > item_a_state {
                return false;
            };
            match item_defeff_id {
                Some(defeff_id) => defeff_id == a_effect.ae.id,
                _ => false,
            }
        }
        // No additional restrictions for overload effects except for item being overloaded
        ad::AState::Overload => item_a_state >= a_effect.ae.state,
    }
}
