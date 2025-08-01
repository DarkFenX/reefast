use crate::{
    def::{AttrVal, OF},
    misc::Spool,
    nd::NRemoteRepGetter,
    rd::REffectKey,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptionReload, CycleOptions, get_item_cycle_info},
        vast::{StatTank, VastFitData},
    },
    ud::UItemKey,
    util::RMapRMap,
};

impl VastFitData {
    pub(in crate::svc) fn get_stat_remote_rps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        spool: Option<Spool>,
    ) -> StatTank<AttrVal> {
        StatTank {
            shield: get_orrps(ctx, calc, spool, &self.orr_shield),
            armor: get_orrps(ctx, calc, spool, &self.orr_armor),
            hull: get_orrps(ctx, calc, spool, &self.orr_hull),
        }
    }
    pub(in crate::svc) fn get_stat_remote_cps(&self, ctx: SvcCtx, calc: &mut Calc) -> AttrVal {
        get_orrps(ctx, calc, None, &self.orr_cap)
    }
}

const ORR_CYCLE_OPTIONS: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Burst,
    reload_optionals: false,
};

fn get_orrps(
    ctx: SvcCtx,
    calc: &mut Calc,
    spool: Option<Spool>,
    fit_data: &RMapRMap<UItemKey, REffectKey, NRemoteRepGetter>,
) -> AttrVal {
    let mut rps = OF(0.0);
    for (&item_key, item_data) in fit_data.iter() {
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, ORR_CYCLE_OPTIONS, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        for (&effect_key, rep_getter) in item_data.iter() {
            let r_effect = ctx.u_data.src.get_effect(effect_key);
            let output_per_cycle = match rep_getter(ctx, calc, item_key, r_effect, spool, None) {
                Some(output_per_cycle) => output_per_cycle,
                None => continue,
            };
            let effect_cycles = match cycle_map.get(&effect_key) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            rps += output_per_cycle.get_total() / effect_cycles.get_average_cycle_time();
        }
    }
    rps
}
