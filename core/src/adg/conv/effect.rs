use std::hash::Hash;

use crate::{
    ad,
    adg::{GData, GSupport},
    defs::{EAttrId, EEffectId, EItemGrpId, EItemId},
    ec, ed,
    util::{StMap, StSet, StrMsgError},
};

impl ed::EFighterAbil {
    fn get_disallow_hisec(&self) -> bool {
        self.disallow_hisec
    }
    fn get_disallow_lowsec(&self) -> bool {
        self.disallow_lowsec
    }
}

pub(in crate::adg::conv) fn conv_effects(g_data: &GData, g_supp: &GSupport) -> Vec<ad::AEffect> {
    let mut a_effects = Vec::new();
    for e_effect in g_data.effects.iter() {
        let state = match e_effect.category_id {
            ec::effcats::PASSIVE => ad::AState::Offline,
            ec::effcats::ACTIVE => ad::AState::Active,
            ec::effcats::TARGET => ad::AState::Active,
            ec::effcats::ONLINE => ad::AState::Online,
            ec::effcats::OVERLOAD => ad::AState::Overload,
            ec::effcats::SYSTEM => ad::AState::Offline,
            _ => {
                let msg = format!("{} uses unknown effect category {}", e_effect, e_effect.category_id);
                tracing::warn!("{msg}");
                continue;
            }
        };
        let mut a_effect = ad::AEffect::new(
            e_effect.id,
            e_effect.category_id,
            state,
            e_effect.is_assistance,
            e_effect.is_offensive,
            None,
            None,
            e_effect.discharge_attr_id,
            e_effect.duration_attr_id,
            e_effect.range_attr_id,
            e_effect.falloff_attr_id,
            e_effect.tracking_attr_id,
            e_effect.usage_chance_attr_id,
            e_effect.resist_attr_id,
            ad::AEffectModBuildStatus::Unbuilt,
            Vec::new(),
            Vec::new(),
            g_supp.eff_buff_map.get(&e_effect.id).cloned(),
            g_supp.eff_charge_map.get(&e_effect.id).copied(),
        );
        let mut mod_errs = 0;
        for e_modifier in e_effect.mods.iter() {
            // Process effect stoppers first
            match extract_stopper(e_modifier) {
                Ok(Some(effect_id)) => {
                    if !a_effect.stop_ids.contains(&effect_id) {
                        a_effect.stop_ids.push(effect_id)
                    };
                    continue;
                }
                Err(e) => {
                    let msg = format!("failed to build stopper for {}: {}", a_effect, e);
                    tracing::warn!("{msg}");
                    mod_errs += 1;
                    continue;
                }
                _ => (),
            }
            // Process regular attribute modifiers
            let a_mod_res = match e_modifier.func.as_str() {
                "ItemModifier" => conv_item_mod(e_modifier, &a_effect),
                "LocationModifier" => conv_loc_mod(e_modifier, &a_effect),
                "LocationGroupModifier" => conv_locgrp_mod(e_modifier, &a_effect),
                "LocationRequiredSkillModifier" => conv_locsrq_mod(e_modifier, &a_effect),
                "OwnerRequiredSkillModifier" => conv_ownsrq_mod(e_modifier, &a_effect),
                _ => Err(StrMsgError::new(format!("unknown function \"{}\"", e_modifier.func))),
            };
            match a_mod_res {
                Ok(a_mod) => a_effect.mods.push(a_mod),
                Err(e) => {
                    let msg = format!("failed to build modifier for {}: {}", a_effect, e);
                    tracing::warn!("{msg}");
                    mod_errs += 1;
                    continue;
                }
            }
        }
        match mod_errs {
            0 => a_effect.mod_build_status = ad::AEffectModBuildStatus::Success,
            _ if !a_effect.mods.is_empty() || !a_effect.stop_ids.is_empty() => {
                a_effect.mod_build_status = ad::AEffectModBuildStatus::SuccessPartial(mod_errs)
            }
            _ => a_effect.mod_build_status = ad::AEffectModBuildStatus::Error(mod_errs),
        }
        a_effects.push(a_effect);
    }
    // Transfer some data from abilities onto effects
    let hisec_ban_map = extract_ability_map(g_data, ed::EFighterAbil::get_disallow_hisec);
    let lowsec_ban_map = extract_ability_map(g_data, ed::EFighterAbil::get_disallow_lowsec);
    for a_effect in a_effects.iter_mut() {
        // Hisec flag
        match hisec_ban_map.get(&a_effect.id) {
            None => (),
            Some(flags) => match flags.len() {
                1 => {
                    a_effect.hisec = Some(!*flags.iter().next().unwrap());
                }
                _ => {
                    let msg = format!(
                        "{} has {} distinct \"disallow in hisec\" values mapped from fighter abilities",
                        a_effect,
                        flags.len()
                    );
                    tracing::warn!("{msg}");
                }
            },
        }
        // Lowsec flag
        match lowsec_ban_map.get(&a_effect.id) {
            None => (),
            Some(flags) => match flags.len() {
                1 => {
                    a_effect.lowsec = Some(!*flags.iter().next().unwrap());
                }
                _ => {
                    let msg = format!(
                        "{} has {} distinct \"disallow in lowsec\" values mapped from fighter abilities",
                        a_effect,
                        flags.len()
                    );
                    tracing::warn!("{msg}");
                }
            },
        }
    }
    a_effects
}

fn extract_stopper(e_modifier: &ed::EEffectMod) -> Result<Option<EEffectId>, StrMsgError> {
    match e_modifier.func.as_str() {
        "EffectStopper" => {
            let domain = get_arg_str(&e_modifier.args, "domain")?;
            if domain.ne("target") {
                return Err(StrMsgError::new(format!("unexpected domain \"{domain}\"")));
            }
            Ok(Some(get_arg_int(&e_modifier.args, "effectID")?))
        }
        _ => Ok(None),
    }
}

fn conv_item_mod(e_modifier: &ed::EEffectMod, a_effect: &ad::AEffect) -> Result<ad::AEffectModifier, StrMsgError> {
    Ok(ad::AEffectModifier::new(
        get_mod_src_attr_id(e_modifier)?,
        get_mod_operation(e_modifier)?,
        ad::AEffectAffecteeFilter::Direct(get_mod_location(e_modifier, a_effect)?),
        get_mod_affectee_attr_id(e_modifier)?,
    ))
}

fn conv_loc_mod(e_modifier: &ed::EEffectMod, a_effect: &ad::AEffect) -> Result<ad::AEffectModifier, StrMsgError> {
    Ok(ad::AEffectModifier::new(
        get_mod_src_attr_id(e_modifier)?,
        get_mod_operation(e_modifier)?,
        ad::AEffectAffecteeFilter::Loc(get_mod_location(e_modifier, a_effect)?),
        get_mod_affectee_attr_id(e_modifier)?,
    ))
}

fn conv_locgrp_mod(e_modifier: &ed::EEffectMod, a_effect: &ad::AEffect) -> Result<ad::AEffectModifier, StrMsgError> {
    Ok(ad::AEffectModifier::new(
        get_mod_src_attr_id(e_modifier)?,
        get_mod_operation(e_modifier)?,
        ad::AEffectAffecteeFilter::LocGrp(get_mod_location(e_modifier, a_effect)?, get_mod_grp_id(e_modifier)?),
        get_mod_affectee_attr_id(e_modifier)?,
    ))
}

fn conv_locsrq_mod(e_modifier: &ed::EEffectMod, a_effect: &ad::AEffect) -> Result<ad::AEffectModifier, StrMsgError> {
    Ok(ad::AEffectModifier::new(
        get_mod_src_attr_id(e_modifier)?,
        get_mod_operation(e_modifier)?,
        ad::AEffectAffecteeFilter::LocSrq(
            get_mod_location(e_modifier, a_effect)?,
            ad::AModifierSrq::ItemId(get_mod_skill_id(e_modifier)?),
        ),
        get_mod_affectee_attr_id(e_modifier)?,
    ))
}

fn conv_ownsrq_mod(e_modifier: &ed::EEffectMod, a_effect: &ad::AEffect) -> Result<ad::AEffectModifier, StrMsgError> {
    if !matches!(
        get_mod_location(e_modifier, a_effect)?,
        ad::AEffectLocation::Char | ad::AEffectLocation::Target
    ) {
        return Err(StrMsgError::new(format!(
            "unexpected domain \"{}\" for owner-filtered modification",
            get_arg_str(&e_modifier.args, "domain")?
        )));
    }
    Ok(ad::AEffectModifier::new(
        get_mod_src_attr_id(e_modifier)?,
        get_mod_operation(e_modifier)?,
        ad::AEffectAffecteeFilter::OwnSrq(ad::AModifierSrq::ItemId(get_mod_skill_id(e_modifier)?)),
        get_mod_affectee_attr_id(e_modifier)?,
    ))
}

fn get_mod_src_attr_id(e_modifier: &ed::EEffectMod) -> Result<EAttrId, StrMsgError> {
    get_arg_int(&e_modifier.args, "modifyingAttributeID")
}

fn get_mod_affectee_attr_id(e_modifier: &ed::EEffectMod) -> Result<EAttrId, StrMsgError> {
    get_arg_int(&e_modifier.args, "modifiedAttributeID")
}

fn get_mod_location(e_modifier: &ed::EEffectMod, a_effect: &ad::AEffect) -> Result<ad::AEffectLocation, StrMsgError> {
    let domain = get_arg_str(&e_modifier.args, "domain")?;
    match domain.as_str() {
        "itemID" => Ok(ad::AEffectLocation::Item),
        "charID" => Ok(ad::AEffectLocation::Char),
        "shipID" => Ok(ad::AEffectLocation::Ship),
        "structureID" => Ok(ad::AEffectLocation::Structure),
        "targetID" => match a_effect.category {
            ec::effcats::TARGET => Ok(ad::AEffectLocation::Target),
            _ => Err(StrMsgError::new(format!(
                "modifier uses {} domain on untargeted effect",
                domain
            ))),
        },
        "otherID" => Ok(ad::AEffectLocation::Other),
        _ => Err(StrMsgError::new(format!("unknown domain {domain}"))),
    }
}

fn get_mod_operation(e_modifier: &ed::EEffectMod) -> Result<ad::AOp, StrMsgError> {
    let op = get_arg_int(&e_modifier.args, "operation")?;
    match op {
        -1 => Ok(ad::AOp::PreAssign),
        0 => Ok(ad::AOp::PreMul),
        1 => Ok(ad::AOp::PreDiv),
        2 => Ok(ad::AOp::Add),
        3 => Ok(ad::AOp::Sub),
        4 => Ok(ad::AOp::PostMul),
        5 => Ok(ad::AOp::PostDiv),
        6 => Ok(ad::AOp::PostPerc),
        7 => Ok(ad::AOp::PostAssign),
        _ => Err(StrMsgError::new(format!("unknown operation {op}"))),
    }
}

fn get_mod_grp_id(e_modifier: &ed::EEffectMod) -> Result<EItemGrpId, StrMsgError> {
    get_arg_int(&e_modifier.args, "groupID")
}

fn get_mod_skill_id(e_modifier: &ed::EEffectMod) -> Result<EItemId, StrMsgError> {
    get_arg_int(&e_modifier.args, "skillTypeID")
}

fn get_arg_int(args: &StMap<String, ed::EPrimitive>, name: &str) -> Result<i32, StrMsgError> {
    let primitive = args
        .get(name)
        .ok_or(StrMsgError::new(format!("no \"{name}\" in args")))?;
    match primitive {
        ed::EPrimitive::Int(i) => Ok(*i),
        _ => Err(StrMsgError::new(format!("expected int in \"{name}\" value"))),
    }
}

fn get_arg_str(args: &StMap<String, ed::EPrimitive>, name: &str) -> Result<String, StrMsgError> {
    let primitive = args
        .get(name)
        .ok_or(StrMsgError::new(format!("no \"{name}\" in args")))?;
    match primitive {
        ed::EPrimitive::String(s) => Ok(s.into()),
        _ => Err(StrMsgError::new(format!("expected string in \"{name}\" value"))),
    }
}

fn extract_ability_map<F, T>(g_data: &GData, getter: F) -> StMap<EEffectId, StSet<T>>
where
    F: Fn(&ed::EFighterAbil) -> T,
    T: Eq + Hash,
{
    let mut map = StMap::new();
    for e_abil in g_data.abils.iter() {
        match ec::extras::get_abil_effect(e_abil.id) {
            None => continue,
            Some(effect_id) => map
                .entry(effect_id)
                .or_insert_with(|| StSet::new())
                .insert(getter(e_abil)),
        };
    }
    map
}
