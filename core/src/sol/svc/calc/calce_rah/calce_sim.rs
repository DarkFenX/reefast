use std::collections::BTreeMap;

use itertools::Itertools;
use ordered_float::{Float, OrderedFloat as OF};

use crate::{
    sol::{
        AttrVal, DmgKinds, FitId, ItemId,
        svc::calc::{Calc, CalcAttrVal},
        uad::Uad,
    },
    util::{RMap, RSet},
};

use super::{
    rah_data_sim::RahDataSim,
    rah_history_entry::RahSimHistoryEntry,
    rah_info::RahInfo,
    shared::{
        ARMOR_EM_ATTR_ID, ARMOR_EXPL_ATTR_ID, ARMOR_HP_ATTR_ID, ARMOR_KIN_ATTR_ID, ARMOR_THERM_ATTR_ID,
        HULL_HP_ATTR_ID, RAH_EFFECT_ID, RAH_SHIFT_ATTR_ID, SHIELD_HP_ATTR_ID, TickCount, get_fit_rah_incoming_dps,
        rah_round,
    },
    ship_stats::RahShipStats,
    tick_iter::RahSimTickIter,
};

impl Calc {
    pub(super) fn rah_run_simulation(&mut self, uad: &Uad, fit_id: &FitId) {
        let fit = uad.fits.get_fit(fit_id).unwrap();
        let ship_id = match fit.ship {
            Some(ship_id) => ship_id,
            None => {
                // Since there were no calculated values stored in sim prior to simulation, and we
                // are setting unadapted values - effectively values of resonances do not change,
                // and no updates needed
                self.set_fit_rahs_unadapted(uad, fit_id, false);
                return;
            }
        };
        // Keys in this map have to be sorted, since it defines RAH order in simulation history,
        // which hashes vectors with history entries
        let mut sim_datas = self.get_fit_rah_sim_datas(uad, fit_id);
        // If the map is empty, no setting fallbacks needed, they were set in the data getter
        if sim_datas.is_empty() {
            return;
        }
        let dps_profile = get_fit_rah_incoming_dps(uad, fit);
        let mut history_entries_seen = RSet::new();
        let mut sim_history = Vec::new();
        // Run "zero" simulation tick - write initial results and record initial state in history
        let mut sim_history_entry = Vec::with_capacity(sim_datas.len());
        for (item_id, item_sim_data) in sim_datas.iter() {
            // Sets unadapted values, since info contains unadapted resonance values
            self.set_rah_result(uad, item_id, item_sim_data.info.resos, false);
            // Round resonances for the zero history tick. Later they will be rounded by function
            // which adapts resonances to damage
            let item_history_entry = RahSimHistoryEntry::new(*item_id, OF(0.0), &item_sim_data.info.resos, true);
            sim_history_entry.push(item_history_entry);
        }
        history_entries_seen.insert(sim_history_entry.clone());
        sim_history.push(sim_history_entry);
        // Run simulation
        for tick_data in RahSimTickIter::new(sim_datas.iter()) {
            // For each RAH, calculate damage received during this tick
            let ship_stats = match self.get_ship_stats(uad, &ship_id) {
                Some(ship_stats) => ship_stats,
                None => {
                    for item_id in sim_datas.keys() {
                        // Any issues with ship resonance fetch should happen on the very first sim
                        // tick, so results should coincide to default state
                        self.set_rah_unadapted(uad, item_id, false);
                    }
                    return;
                }
            };
            for item_sim_data in sim_datas.values_mut() {
                item_sim_data.taken_dmg.em += dps_profile.get_em() * ship_stats.resos.em * tick_data.time_passed;
                item_sim_data.taken_dmg.thermal +=
                    dps_profile.get_thermal() * ship_stats.resos.thermal * tick_data.time_passed;
                item_sim_data.taken_dmg.kinetic +=
                    dps_profile.get_kinetic() * ship_stats.resos.kinetic * tick_data.time_passed;
                item_sim_data.taken_dmg.explosive +=
                    dps_profile.get_explosive() * ship_stats.resos.explosive * tick_data.time_passed;
                if let Some(breacher) = dps_profile.get_breacher() {
                    let breacher_dps = Float::min(
                        breacher.get_absolute_max(),
                        breacher.get_relative_max() * ship_stats.total_hp,
                    );
                    // Breacher counts as EM damage for some reason
                    item_sim_data.taken_dmg.em += breacher_dps * tick_data.time_passed;
                }
            }
            // If RAH just finished its cycle, make resist switch
            for cycled_item_id in tick_data.cycled {
                let item_sim_data = sim_datas.get_mut(&cycled_item_id).unwrap();
                let mut taken_dmg = DmgKinds {
                    em: OF(0.0),
                    thermal: OF(0.0),
                    kinetic: OF(0.0),
                    explosive: OF(0.0),
                };
                // Extract damage ship taken during RAH cycle, replacing it with 0's
                std::mem::swap(&mut taken_dmg, &mut item_sim_data.taken_dmg);
                let next_resos = get_next_resonances(
                    self.rah.resonances.get(&cycled_item_id).unwrap().unwrap(),
                    taken_dmg,
                    item_sim_data.info.shift_amount,
                );
                // Write new resonances to results, letting everyone know about the changes. This is
                // needed to get updated ship resonances next tick.
                self.set_rah_result(uad, &cycled_item_id, next_resos, true);
            }
            // Compose history entry of current tick
            let mut sim_history_entry = Vec::with_capacity(sim_datas.len());
            for item_id in sim_datas.keys() {
                let item_cycling_time = *tick_data.cycling_times.get(item_id).unwrap();
                let item_resos = self.rah.resonances.get(item_id).unwrap().unwrap();
                let item_history_entry = RahSimHistoryEntry::new(*item_id, item_cycling_time, &item_resos, false);
                sim_history_entry.push(item_history_entry);
            }
            // See if we're in a loop, if we are - calculate average resists across tick states
            // which are within the loop
            if history_entries_seen.contains(&sim_history_entry) {
                // If there was no need to adapt (= sim history contains only zero tick data), set
                // unadapted resonances as results to avoid unnecessary for this case rounding.
                // Normal process uses history values, which contains rounded resonances
                if sim_history.len() <= 1 {
                    for (item_id, item_sim_data) in sim_datas.iter() {
                        self.set_rah_result(uad, item_id, item_sim_data.info.resos, false);
                    }
                    return;
                }
                let index = sim_history.iter().position(|v| v == &sim_history_entry).unwrap();
                let avg_resos = get_average_resonances(&sim_history[index..]);
                self.set_partial_fit_rahs_result(uad, avg_resos, &sim_datas);
                return;
            }
            // No loop - update history
            history_entries_seen.insert(sim_history_entry.clone());
            sim_history.push(sim_history_entry);
        }
        // If we didn't find any RAH state loops during specified count of sim ticks, calculate
        // average resonances based on whole history, excluding initial adaptation period
        let ticks_to_ignore = estimate_initial_adaptation_ticks(&sim_datas, &sim_history);
        // Never ignore more than half of the history
        let ticks_to_ignore = ticks_to_ignore.min(sim_history.len() / 2);
        let avg_resos = get_average_resonances(&sim_history[ticks_to_ignore..]);
        self.set_partial_fit_rahs_result(uad, avg_resos, &sim_datas);
    }
    fn get_ship_stats(&mut self, uad: &Uad, ship_id: &ItemId) -> Option<RahShipStats> {
        let em = self.get_item_attr_val_full(uad, ship_id, &ARMOR_EM_ATTR_ID).ok()?.dogma;
        let thermal = self
            .get_item_attr_val_full(uad, ship_id, &ARMOR_THERM_ATTR_ID)
            .ok()?
            .dogma;
        let kinetic = self
            .get_item_attr_val_full(uad, ship_id, &ARMOR_KIN_ATTR_ID)
            .ok()?
            .dogma;
        let explosive = self
            .get_item_attr_val_full(uad, ship_id, &ARMOR_EXPL_ATTR_ID)
            .ok()?
            .dogma;
        let shield_hp = match self.get_item_attr_val_full(uad, ship_id, &SHIELD_HP_ATTR_ID) {
            Ok(shield_hp) => shield_hp.dogma,
            Err(_) => OF(0.0),
        };
        let armor_hp = match self.get_item_attr_val_full(uad, ship_id, &ARMOR_HP_ATTR_ID) {
            Ok(armor_hp) => armor_hp.dogma,
            Err(_) => OF(0.0),
        };
        let hull_hp = match self.get_item_attr_val_full(uad, ship_id, &HULL_HP_ATTR_ID) {
            Ok(hull_hp) => hull_hp.dogma,
            Err(_) => OF(0.0),
        };
        Some(RahShipStats {
            resos: DmgKinds {
                em,
                thermal,
                kinetic,
                explosive,
            },
            total_hp: shield_hp + armor_hp + hull_hp,
        })
    }
    fn get_fit_rah_sim_datas(&mut self, uad: &Uad, fit_id: &FitId) -> BTreeMap<ItemId, RahDataSim> {
        let mut rah_datas = BTreeMap::new();
        for item_id in self.rah.by_fit.get(fit_id).copied().collect_vec() {
            let rah_attrs = match self.get_rah_sim_data(uad, &item_id) {
                Some(rah_attrs) => rah_attrs,
                // Whenever a RAH has unacceptable for sim attributes, set unadapted values and
                // don't add it to the map. No updates needed, since this method should be called
                // before sim makes any changes
                None => {
                    self.set_rah_unadapted(uad, &item_id, false);
                    continue;
                }
            };
            rah_datas.insert(item_id, rah_attrs);
        }
        rah_datas
    }
    fn get_rah_sim_data(&mut self, uad: &Uad, item_id: &ItemId) -> Option<RahDataSim> {
        // Get resonances through postprocessing functions, since we already installed them for RAHs
        let res_em = self.get_item_attr_val_no_pp(uad, item_id, &ARMOR_EM_ATTR_ID).ok()?;
        let res_therm = self.get_item_attr_val_no_pp(uad, item_id, &ARMOR_THERM_ATTR_ID).ok()?;
        let res_kin = self.get_item_attr_val_no_pp(uad, item_id, &ARMOR_KIN_ATTR_ID).ok()?;
        let res_expl = self.get_item_attr_val_no_pp(uad, item_id, &ARMOR_EXPL_ATTR_ID).ok()?;
        if res_em.dogma == OF(1.0)
            && res_therm.dogma == OF(1.0)
            && res_kin.dogma == OF(1.0)
            && res_expl.dogma == OF(1.0)
        {
            return None;
        }
        // Other attributes using regular getters
        // Divide by 100 for convenience - raw form of shift amount is defined in percentages, while
        // resonances are in absolute form
        let shift_amount = self
            .get_item_attr_val_full(uad, item_id, &RAH_SHIFT_ATTR_ID)
            .ok()?
            .dogma
            / OF(100.0);
        if shift_amount <= OF(0.0) {
            return None;
        }
        // Raw form of cycle time is defined in milliseconds, convert into seconds
        let cycle_s = self.get_item_effect_id_duration(uad, item_id, &RAH_EFFECT_ID)? / OF(1000.0);
        if cycle_s <= OF(0.0) {
            return None;
        }
        let rah_info = RahInfo::new(res_em, res_therm, res_kin, res_expl, cycle_s, shift_amount);
        Some(RahDataSim::new(rah_info))
    }
    // Set resonances to unadapted values in sim storage for all RAHs of requested fit
    fn set_fit_rahs_unadapted(&mut self, uad: &Uad, fit_id: &FitId, notify: bool) {
        for item_id in self.rah.by_fit.get(fit_id).copied().collect_vec() {
            self.set_rah_unadapted(uad, &item_id, notify);
        }
    }
    fn set_rah_unadapted(&mut self, uad: &Uad, item_id: &ItemId, notify: bool) {
        let em = self
            .get_item_attr_val_no_pp(uad, item_id, &ARMOR_EM_ATTR_ID)
            .unwrap_or(CalcAttrVal {
                base: OF(1.0),
                dogma: OF(1.0),
                extra: OF(1.0),
            });
        let thermal = self
            .get_item_attr_val_no_pp(uad, item_id, &ARMOR_THERM_ATTR_ID)
            .unwrap_or(CalcAttrVal {
                base: OF(1.0),
                dogma: OF(1.0),
                extra: OF(1.0),
            });
        let kinetic = self
            .get_item_attr_val_no_pp(uad, item_id, &ARMOR_KIN_ATTR_ID)
            .unwrap_or(CalcAttrVal {
                base: OF(1.0),
                dogma: OF(1.0),
                extra: OF(1.0),
            });
        let explosive = self
            .get_item_attr_val_no_pp(uad, item_id, &ARMOR_EXPL_ATTR_ID)
            .unwrap_or(CalcAttrVal {
                base: OF(1.0),
                dogma: OF(1.0),
                extra: OF(1.0),
            });
        let rah_resos = DmgKinds {
            em,
            thermal,
            kinetic,
            explosive,
        };
        self.set_rah_result(uad, item_id, rah_resos, notify);
    }
    // Result application methods
    fn set_rah_result(&mut self, uad: &Uad, item_id: &ItemId, resos: DmgKinds<CalcAttrVal>, notify: bool) {
        self.rah.resonances.get_mut(item_id).unwrap().replace(resos);
        if notify {
            self.force_attr_postproc_recalc(uad, item_id, &ARMOR_EM_ATTR_ID);
            self.force_attr_postproc_recalc(uad, item_id, &ARMOR_THERM_ATTR_ID);
            self.force_attr_postproc_recalc(uad, item_id, &ARMOR_KIN_ATTR_ID);
            self.force_attr_postproc_recalc(uad, item_id, &ARMOR_EXPL_ATTR_ID);
        }
    }
    fn set_partial_fit_rahs_result(
        &mut self,
        uad: &Uad,
        resos: RMap<ItemId, DmgKinds<AttrVal>>,
        sim_datas: &BTreeMap<ItemId, RahDataSim>,
    ) {
        for (item_id, item_sim_data) in sim_datas.iter() {
            // Average resonance is what passed as resonances for this method; average resonance
            // getter might not return resonances for all RAHs, and it's hard to trace when/why this
            // might happen. For safety, just use unadapted values if that happens
            let item_resos = match resos.get(item_id) {
                Some(item_avg_resos) => DmgKinds {
                    em: CalcAttrVal {
                        base: item_sim_data.info.resos.em.base,
                        dogma: item_avg_resos.em,
                        extra: item_avg_resos.em,
                    },
                    thermal: CalcAttrVal {
                        base: item_sim_data.info.resos.thermal.base,
                        dogma: item_avg_resos.thermal,
                        extra: item_avg_resos.thermal,
                    },
                    kinetic: CalcAttrVal {
                        base: item_sim_data.info.resos.kinetic.base,
                        dogma: item_avg_resos.kinetic,
                        extra: item_avg_resos.kinetic,
                    },
                    explosive: CalcAttrVal {
                        base: item_sim_data.info.resos.explosive.base,
                        dogma: item_avg_resos.explosive,
                        extra: item_avg_resos.explosive,
                    },
                },
                None => item_sim_data.info.resos,
            };
            self.set_rah_result(uad, item_id, item_resos, true)
        }
    }
}

fn get_next_resonances(
    mut resonances: DmgKinds<CalcAttrVal>,
    taken_dmg: DmgKinds<AttrVal>,
    shift_amount: AttrVal,
) -> DmgKinds<CalcAttrVal> {
    // Rounding in this function to avoid float errors serves two purposes:
    // 1) it helps in history loop detection;
    // 2) it helps to avoid weird results in unrealistic edge cases, e.g. RAH which starts 0/0/100/0
    // and shifts towards 0/100/0/0 with steps of 10.
    // We borrow resistances from at least 2 resist types, possibly more if ship didn't take any
    // damage of those types
    let donors = taken_dmg.iter().filter(|v| **v <= OF(0.0)).count().max(2);
    let recipients = 4 - donors;
    // There can be 4 donors (and thus 0 recipients) in case no damage is received, which can happen
    // when resists reach 100% / resonance reaches 0
    if recipients == 0 {
        return resonances;
    }
    let recipients = AttrVal::from(recipients as u8);
    // Indices are against damage type container, i.e. order is EM, explosive, kinetic, thermal.
    // When equal damage is received across several damage types, those which come earlier in this
    // list will be picked as donors. In EVE, it's this way probably due to backing attribute IDs,
    // since the list is in attribute ID ascending order.
    let mut sorted_indices: [usize; 4] = [0, 3, 2, 1];
    sorted_indices.sort_by_key(|v| taken_dmg[*v]); // This sort has to be stable
    let mut total_transferred = OF(0.0);
    // Donate
    for index in sorted_indices[..donors].iter() {
        let current_value = resonances[*index];
        // Can't borrow more than it has
        let to_donate = rah_round(Float::min(shift_amount, OF(1.0) - current_value.dogma));
        total_transferred += to_donate;
        let new_value = rah_round(current_value.dogma + to_donate);
        resonances[*index] = CalcAttrVal {
            base: current_value.base,
            dogma: new_value,
            extra: new_value,
        };
    }
    // Distribute
    let mut to_distribute = total_transferred;
    for index in sorted_indices.iter().rev() {
        let current_value = resonances[*index];
        // Can't give more than set threshold, more than we have, and more than target res can take
        let to_take = [
            rah_round(total_transferred / recipients),
            current_value.dogma,
            to_distribute,
        ]
        .into_iter()
        .min()
        .unwrap();
        to_distribute -= to_take;
        let new_value = rah_round(current_value.dogma - to_take);
        resonances[*index] = CalcAttrVal {
            base: current_value.base,
            dogma: new_value,
            extra: new_value,
        };
        if to_distribute <= OF(0.0) {
            break;
        }
    }
    resonances
}

fn get_average_resonances(sim_history: &[Vec<RahSimHistoryEntry>]) -> RMap<ItemId, DmgKinds<AttrVal>> {
    let mut resos_used = RMap::new();
    for sim_history_entry in sim_history {
        for item_history_entry in sim_history_entry {
            // Add resonances to container only when RAH cycle is just starting
            if item_history_entry.cycling_time_rounded == OF(0.0) {
                resos_used
                    .entry(item_history_entry.item_id)
                    .or_insert_with(Vec::new)
                    .push(item_history_entry.resonances);
            }
        }
    }
    let mut avg_resos = RMap::with_capacity(resos_used.len());
    for (item_id, resos) in resos_used.into_iter() {
        let reso_len = resos.len() as f64;
        let item_avg_resos = match resos.into_iter().reduce(|a, v| DmgKinds {
            em: a.em + v.em,
            thermal: a.thermal + v.thermal,
            kinetic: a.kinetic + v.kinetic,
            explosive: a.explosive + v.explosive,
        }) {
            Some(sum) => DmgKinds {
                em: sum.em / reso_len,
                thermal: sum.thermal / reso_len,
                kinetic: sum.kinetic / reso_len,
                explosive: sum.explosive / reso_len,
            },
            // Should happen when resonance container is empty
            None => continue,
        };
        avg_resos.insert(item_id, item_avg_resos);
    }
    avg_resos
}

fn estimate_initial_adaptation_ticks(
    sim_datas: &BTreeMap<ItemId, RahDataSim>,
    sim_history: &[Vec<RahSimHistoryEntry>],
) -> TickCount {
    // Get count of cycles it takes for each RAH to exhaust its highest resistance
    let mut exhaustion_cycles = RMap::new();
    for (item_id, item_sim_data) in sim_datas.iter() {
        let min_reso = *[
            item_sim_data.info.resos.em.dogma,
            item_sim_data.info.resos.thermal.dogma,
            item_sim_data.info.resos.kinetic.dogma,
            item_sim_data.info.resos.explosive.dogma,
        ]
        .iter()
        .min()
        .unwrap();
        let item_exhaustion_cycles = ((OF(1.0) - min_reso) / item_sim_data.info.shift_amount).ceil();
        exhaustion_cycles.insert(*item_id, item_exhaustion_cycles);
    }
    // Slowest RAH is the one which takes the most time to exhaust its highest resistance when it's
    // used strictly as donor
    let slowest_item_id = sim_datas
        .iter()
        .max_by_key(|(k, v)| AttrVal::from(*exhaustion_cycles.get(k).unwrap()) * v.info.cycle_time)
        .map(|v| *v.0)
        .unwrap();
    // Multiply count of resistance exhaustion cycles by 1.5, to give RAH more time for 'finer'
    // adjustments
    let slowest_cycles = (exhaustion_cycles.get(&slowest_item_id).unwrap() * OF(1.5))
        .ceil()
        .into_inner() as TickCount;
    if slowest_cycles == 0 {
        return 0;
    }
    // We rely on cycling time attribute to be zero in order to determine that cycle for the slowest
    // RAH has just ended. It is zero for the very first tick in the history too, thus we skip it,
    // but take it into initial tick count
    let ignored_tick_count = 1;
    let mut tick_count = ignored_tick_count;
    let mut cycle_count = 0;
    for sim_history_entry in sim_history[ignored_tick_count..].iter() {
        // Once slowest RAH finished last cycle, do not count this tick and break the loop
        for item_history_entry in sim_history_entry.iter() {
            if item_history_entry.item_id == slowest_item_id {
                if item_history_entry.cycling_time_rounded == OF(0.0) {
                    cycle_count += 1;
                }
                break;
            }
        }
        if cycle_count >= slowest_cycles {
            break;
        }
        tick_count += 1;
    }
    tick_count
}
