use either::Either;

use super::info::{Cycle, CycleSimple};
use crate::{
    def::OF,
    rd::REffectKey,
    svc::{SvcCtx, calc::Calc, eff_funcs},
    ud::{UDrone, UItemKey},
    util::{InfCount, RMap},
};

pub(super) fn get_drone_cycle_info(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    drone: &UDrone,
    ignore_state: bool,
) -> Option<RMap<REffectKey, Cycle>> {
    if !drone.is_loaded() {
        return None;
    };
    let mut cycle_infos = RMap::new();
    let effect_keys = match ignore_state {
        true => Either::Left(drone.get_effect_datas().unwrap().keys().copied()),
        false => Either::Right(drone.get_reffs().unwrap().iter().copied()),
    };
    for effect_key in effect_keys {
        let effect = ctx.u_data.src.get_effect(effect_key);
        if !effect.is_active_with_duration() {
            continue;
        }
        let duration_s = match eff_funcs::get_effect_duration_s(ctx, calc, item_key, effect) {
            Some(duration_s) => duration_s,
            None => continue,
        };
        // Assume all drone effects just repeat themselves - ignoring all settings, self-destruction
        // flags, limited charges & reloads
        cycle_infos.insert(
            effect_key,
            Cycle::Simple(CycleSimple {
                active_time: duration_s,
                inactive_time: OF(0.0),
                repeat_count: InfCount::Infinite,
            }),
        );
    }
    Some(cycle_infos)
}
