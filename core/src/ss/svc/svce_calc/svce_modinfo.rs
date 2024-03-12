use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{
    ad,
    defs::{EAttrId, SsItemId},
    ss::{
        svc::{
            svce_calc::mod_info::{ModOpInfo, ModSrcInfo, ModSrcValInfo},
            SsSvcs,
        },
        SsView,
    },
    util::Result,
};

use super::{mod_info::ModInfo, svce_attr::is_penalizable};

impl SsSvcs {
    // Query methods
    pub(in crate::ss) fn calc_get_item_mods(
        &mut self,
        ss_view: &SsView,
        item_id: &SsItemId,
    ) -> Result<HashMap<EAttrId, Vec<ModInfo>>> {
        let item = ss_view.items.get_item(item_id)?;
        let mut info_map = HashMap::new();
        for attr_id in self.calc_get_item_attr_ids(ss_view, item_id)? {
            let attr = match ss_view.src.get_a_attr(&attr_id) {
                Some(attr) => attr,
                None => continue,
            };
            let mut infos = Vec::new();
            for (mod_key, modification) in self.calc_get_modifications(ss_view, item, &attr_id) {
                let mut srcs = Vec::with_capacity(1);
                if let Some(src_attr_id) = mod_key.src_attr_id {
                    let src = ModSrcInfo::new(mod_key.src_item_id, ModSrcValInfo::AttrId(src_attr_id));
                    srcs.push(src);
                };
                let info = ModInfo::new(
                    modification.val,
                    (&modification.op).into(),
                    is_penalizable(&modification, &attr),
                    modification.aggr_mode,
                    srcs,
                );
                infos.push(info);
            }
            filter_useless(&attr_id, &mut infos, ss_view);
            if !infos.is_empty() {
                info_map.insert(attr_id, infos);
            }
        }
        Ok(info_map)
    }
    // Private methods
    fn calc_get_item_attr_ids(&self, ss_view: &SsView, item_id: &SsItemId) -> Result<HashSet<EAttrId>> {
        let mut attr_ids = HashSet::new();
        for attr_id in ss_view.items.get_item(item_id)?.get_orig_attrs()?.keys() {
            attr_ids.insert(*attr_id);
        }
        for attr_id in self.calc_data.attrs.get_item_attrs(item_id)?.keys() {
            attr_ids.insert(*attr_id);
        }
        Ok(attr_ids)
    }
}

fn filter_useless(attr_id: &EAttrId, mods: &mut Vec<ModInfo>, ss_view: &SsView) {
    // Filter out modifications which get overridden by post-assigment
    filter_pre_postassign(mods);
    // Filter out modifications where right hand operand doesn't do anything because of its value
    filter_neutral_invalid_operands(mods);
    // Since only one of assignment operations is effective, include only that one
    if let Some(attr) = ss_view.src.get_a_attr(attr_id) {
        filter_ineffective_assigns(mods, &attr, ModOpInfo::PreAssign);
        filter_ineffective_assigns(mods, &attr, ModOpInfo::PostAssign);
    }
}

fn filter_pre_postassign(mods: &mut Vec<ModInfo>) {
    if mods.iter().any(|v| matches!(v.op, ModOpInfo::PostAssign)) {
        mods.retain(|m| match m.op {
            // Only those 2 modifications are processed after post-assignment
            ModOpInfo::PostAssign | ModOpInfo::Limit | ModOpInfo::ExtraMul => true,
            _ => false,
        });
    };
}

fn filter_neutral_invalid_operands(mods: &mut Vec<ModInfo>) {
    mods.retain(|m| match m.op {
        ModOpInfo::PreMul | ModOpInfo::PostMul | ModOpInfo::ExtraMul => m.val != 1.0,
        ModOpInfo::PreDiv | ModOpInfo::PostDiv => m.val != 1.0 && m.val != 0.0,
        ModOpInfo::Add | ModOpInfo::Sub | ModOpInfo::PostPerc => m.val != 0.0,
        _ => true,
    });
}

fn filter_ineffective_assigns(mods: &mut Vec<ModInfo>, attr: &ad::AAttr, op: ModOpInfo) {
    let assign_mods = mods.extract_if(|m| op == m.op).collect_vec();
    if !assign_mods.is_empty() {
        let effective_mod = match attr.hig {
            true => assign_mods.into_iter().max_by(|a, b| a.val.total_cmp(&b.val)).unwrap(),
            false => assign_mods.into_iter().min_by(|a, b| a.val.total_cmp(&b.val)).unwrap(),
        };
        mods.push(effective_mod);
    }
}
