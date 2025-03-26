use itertools::Itertools;

use crate::{
    adg::{
        EData, GSupport,
        rels::{KeyDb, Pk},
    },
    ec,
    util::{Named, StSet, StrMsgError},
};

const MAX_CYCLES: u32 = 100;

pub(in crate::adg) fn clean_unused(alive: &mut EData, g_supp: &GSupport) -> Result<(), StrMsgError> {
    let mut trash = EData::new();
    trash_all(alive, &mut trash);
    restore_core_items(alive, &mut trash, g_supp);
    restore_hardcoded_attrs(alive, &mut trash);
    restore_hardcoded_buffs(alive, &mut trash);

    let mut counter = 0;
    let mut changes = true;
    while changes {
        counter += 1;
        if counter > MAX_CYCLES {
            let msg = format!("reached limit of {MAX_CYCLES} cycles during cleanup");
            tracing::error!("{msg}");
            return Err(StrMsgError::new(msg));
        }
        changes = restore_item_data(alive, &mut trash) || restore_fk_tgts(alive, &mut trash, g_supp);
    }
    cleanup_report(alive, &trash);
    Ok(())
}

fn move_data<T, F>(src_vec: &mut Vec<T>, dst_vec: &mut Vec<T>, filter: F) -> bool
where
    F: FnMut(&mut T) -> bool,
{
    let drained = src_vec.extract_if(.., filter).collect_vec();
    let changes = !drained.is_empty();
    dst_vec.extend(drained);
    changes
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Initial preparation functions
////////////////////////////////////////////////////////////////////////////////////////////////////
fn trash_all(alive: &mut EData, trash: &mut EData) {
    move_data(&mut alive.items, &mut trash.items, |_| true);
    move_data(&mut alive.groups, &mut trash.groups, |_| true);
    move_data(&mut alive.attrs, &mut trash.attrs, |_| true);
    move_data(&mut alive.item_attrs, &mut trash.item_attrs, |_| true);
    move_data(&mut alive.effects, &mut trash.effects, |_| true);
    move_data(&mut alive.item_effects, &mut trash.item_effects, |_| true);
    move_data(&mut alive.abils, &mut trash.abils, |_| true);
    move_data(&mut alive.item_abils, &mut trash.item_abils, |_| true);
    move_data(&mut alive.buffs, &mut trash.buffs, |_| true);
    move_data(&mut alive.space_comps, &mut trash.space_comps, |_| true);
    move_data(&mut alive.item_srqs, &mut trash.item_srqs, |_| true);
    move_data(&mut alive.muta_items, &mut trash.muta_items, |_| true);
    move_data(&mut alive.muta_attrs, &mut trash.muta_attrs, |_| true);
}

fn restore_core_items(alive: &mut EData, trash: &mut EData, g_supp: &GSupport) {
    let cats = [
        ec::itemcats::CHARGE,
        ec::itemcats::DRONE,
        ec::itemcats::FIGHTER,
        ec::itemcats::IMPLANT,
        ec::itemcats::MODULE,
        ec::itemcats::SHIP,
        ec::itemcats::SKILL,
        ec::itemcats::STRUCTURE,
        ec::itemcats::STRUCTURE_MODULE,
        ec::itemcats::SUBSYSTEM,
    ];
    let mut grps = vec![ec::itemgrps::CHARACTER, ec::itemgrps::EFFECT_BEACON];
    for (&grp, cat) in g_supp.grp_cat_map.iter() {
        if cats.contains(cat) {
            grps.push(grp);
        }
    }
    move_data(&mut trash.items, &mut alive.items, |v| grps.contains(&v.group_id));
}

fn restore_hardcoded_attrs(alive: &mut EData, trash: &mut EData) {
    // Gate scramble strength: isn't defined anywhere, default value is used by HIC WDFG script
    // effects. It is referenced from script WDFG effect modifier infos, but there is no
    // functionality to process modifier infos during cleanup. On top of that, those modifiers are
    // not relied upon - they are replaced with a custom set of modifiers later
    move_data(&mut trash.attrs, &mut alive.attrs, |v| {
        v.id == ec::attrs::GATE_SCRAMBLE_STRENGTH
    });
}

fn restore_hardcoded_buffs(alive: &mut EData, trash: &mut EData) {
    // Used in custom wubble effect
    move_data(&mut trash.buffs, &mut alive.buffs, |v| {
        v.id == ec::buffs::STASIS_WEBIFICATION_BURST
    });
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cyclic restoration functions
////////////////////////////////////////////////////////////////////////////////////////////////////
fn restore_item_data(alive: &mut EData, trash: &mut EData) -> bool {
    let mut item_ids = StSet::new();
    for item in alive.items.iter() {
        item_ids.extend(item.get_pk());
    }
    // We need the data which describes our items directly, so some FKs are avoided deliberately.
    // For instance, having an item-attribute mapping entry restored just because its value refers
    // some item which is already "alive" is undesired.
    //
    // Extra notes on specific entities:
    // - Mutator item conversions are restored for input/output items which are alive
    // - Mutator attribute modifications are restored for alive mutators
    move_data(&mut trash.item_attrs, &mut alive.item_attrs, |v| {
        item_ids.contains(&v.item_id)
    }) || move_data(&mut trash.item_effects, &mut alive.item_effects, |v| {
        item_ids.contains(&v.item_id)
    }) || move_data(&mut trash.item_abils, &mut alive.item_abils, |v| {
        item_ids.contains(&v.item_id)
    }) || move_data(&mut trash.space_comps, &mut alive.space_comps, |v| {
        item_ids.contains(&v.item_id)
    }) || move_data(&mut trash.item_srqs, &mut alive.item_srqs, |v| {
        item_ids.contains(&v.item_id)
    }) || move_data(&mut trash.muta_items, &mut alive.muta_items, |v| {
        item_ids.contains(&v.in_item_id) || item_ids.contains(&v.out_item_id)
    }) || move_data(&mut trash.muta_attrs, &mut alive.muta_attrs, |v| {
        item_ids.contains(&v.muta_id)
    })
}

fn restore_fk_tgts(alive: &mut EData, trash: &mut EData, g_supp: &GSupport) -> bool {
    let fkdb = KeyDb::new_fkdb(alive, g_supp);
    move_data(&mut trash.items, &mut alive.items, |v| fkdb.items.contains(&v.id))
        || move_data(&mut trash.groups, &mut alive.groups, |v| fkdb.groups.contains(&v.id))
        || move_data(&mut trash.attrs, &mut alive.attrs, |v| fkdb.attrs.contains(&v.id))
        || move_data(&mut trash.effects, &mut alive.effects, |v| fkdb.effects.contains(&v.id))
        || move_data(&mut trash.abils, &mut alive.abils, |v| fkdb.abils.contains(&v.id))
        || move_data(&mut trash.buffs, &mut alive.buffs, |v| fkdb.buffs.contains(&v.id))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Reporting
////////////////////////////////////////////////////////////////////////////////////////////////////
fn cleanup_report(alive: &EData, trash: &EData) {
    let cleaned = false;
    let cleaned = vec_report(&alive.items, &trash.items) || cleaned;
    let cleaned = vec_report(&alive.groups, &trash.groups) || cleaned;
    let cleaned = vec_report(&alive.attrs, &trash.attrs) || cleaned;
    let cleaned = vec_report(&alive.item_attrs, &trash.item_attrs) || cleaned;
    let cleaned = vec_report(&alive.effects, &trash.effects) || cleaned;
    let cleaned = vec_report(&alive.item_effects, &trash.item_effects) || cleaned;
    let cleaned = vec_report(&alive.abils, &trash.abils) || cleaned;
    let cleaned = vec_report(&alive.item_abils, &trash.item_abils) || cleaned;
    let cleaned = vec_report(&alive.buffs, &trash.buffs) || cleaned;
    let cleaned = vec_report(&alive.space_comps, &trash.space_comps) || cleaned;
    let cleaned = vec_report(&alive.item_srqs, &trash.item_srqs) || cleaned;
    let cleaned = vec_report(&alive.muta_items, &trash.muta_items) || cleaned;
    let cleaned = vec_report(&alive.muta_attrs, &trash.muta_attrs) || cleaned;
    if !cleaned {
        tracing::info!("no unused data found during cleanup");
    }
}

fn vec_report<T: Named>(alive: &[T], trash: &[T]) -> bool {
    let total = alive.len() + trash.len();
    if total == 0 {
        return false;
    }
    let ratio = trash.len() as f64 / total as f64;
    if ratio > 0.0 {
        tracing::info!("cleaned {:.1}% of {}", ratio * 100.0, T::get_name());
        return true;
    }
    false
}
