use crate::{
    ad, ed,
    util::{RMap, StrMsgError},
};

pub(in crate::adg::flow::conv_pre) fn conv_buffs(e_data: &ed::EData) -> RMap<ad::ABuffId, ad::ABuff> {
    let mut a_buffs = RMap::new();
    for e_buff in e_data.buffs.data.iter() {
        let op = match conv_buff_op(&e_buff.operation) {
            Ok(op) => op,
            Err(e) => {
                let msg = format!("{e_buff}: {e}");
                tracing::warn!("{msg}");
                continue;
            }
        };
        let aggr_mode = match conv_buff_aggr_mode(&e_buff.aggregate_mode) {
            Ok(am) => am,
            Err(e) => {
                let msg = format!("{e_buff}: {e}");
                tracing::warn!("{msg}");
                continue;
            }
        };
        let mut a_mods = Vec::new();
        for e_item_mod in e_buff.item_mods.iter() {
            a_mods.push(ad::ABuffModifier {
                affectee_filter: ad::ABuffAffecteeFilter::Direct,
                affectee_attr_id: e_item_mod.attr_id,
            });
        }
        for e_loc_mod in e_buff.loc_mods.iter() {
            a_mods.push(ad::ABuffModifier {
                affectee_filter: ad::ABuffAffecteeFilter::Loc,
                affectee_attr_id: e_loc_mod.attr_id,
            });
        }
        for e_locgroup_mod in e_buff.locgroup_mods.iter() {
            a_mods.push(ad::ABuffModifier {
                affectee_filter: ad::ABuffAffecteeFilter::LocGrp(e_locgroup_mod.group_id),
                affectee_attr_id: e_locgroup_mod.attr_id,
            });
        }
        for e_locsrq_mod in e_buff.locsrq_mods.iter() {
            a_mods.push(ad::ABuffModifier {
                affectee_filter: ad::ABuffAffecteeFilter::LocSrq(ad::AModifierSrq::ItemId(e_locsrq_mod.skill_id)),
                affectee_attr_id: e_locsrq_mod.attr_id,
            });
        }
        let a_buff = ad::ABuff {
            id: e_buff.id,
            aggr_mode,
            op,
            mods: a_mods,
        };
        a_buffs.insert(a_buff.id, a_buff);
    }
    a_buffs
}

fn conv_buff_aggr_mode(aggr_mode: &str) -> Result<ad::ABuffAggrMode, StrMsgError> {
    match aggr_mode {
        "Minimum" => Ok(ad::ABuffAggrMode::Min),
        "Maximum" => Ok(ad::ABuffAggrMode::Max),
        _ => Err(StrMsgError {
            msg: format!("unexpected aggregate mode \"{aggr_mode}\""),
        }),
    }
}

fn conv_buff_op(operation: &str) -> Result<ad::AOp, StrMsgError> {
    match operation {
        "PreAssignment" => Ok(ad::AOp::PreAssign),
        "PreMul" => Ok(ad::AOp::PreMul),
        "PreDiv" => Ok(ad::AOp::PreDiv),
        "ModAdd" => Ok(ad::AOp::Add),
        "ModSub" => Ok(ad::AOp::Sub),
        "PostMul" => Ok(ad::AOp::PostMul),
        "PostDiv" => Ok(ad::AOp::PostDiv),
        "PostPercent" => Ok(ad::AOp::PostPerc),
        "PostAssignment" => Ok(ad::AOp::PostAssign),
        _ => Err(StrMsgError {
            msg: format!("unexpected operation \"{operation}\""),
        }),
    }
}
