use std::collections::HashMap;
use std::result;

use serde::{de::Error as DeError, Deserialize};

use crate::defines::{ReeFloat, ReeInt};
use crate::dh;

fn into_opt<T: Into<U>, U>(v: Option<T>) -> Option<U> {
    v.map(Into::into)
}
fn into_vec<T: Into<U>, U>(v: Vec<T>) -> Vec<U> {
    v.into_iter().map(|v| v.into()).collect()
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inventory
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize)]
pub(super) struct EveType {
    pub(super) id: ReeInt,
    #[serde(rename = "groupID")]
    pub(super) group_id: ReeInt,
}
impl Into<dh::EveType> for EveType {
    fn into(self) -> dh::EveType {
        dh::EveType::new(self.id, self.group_id)
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct EveGroup {
    pub(super) id: ReeInt,
    #[serde(rename = "categoryID")]
    pub(super) category_id: ReeInt,
}
impl Into<dh::EveGroup> for EveGroup {
    fn into(self) -> dh::EveGroup {
        dh::EveGroup::new(self.id, self.category_id)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Dogma
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize)]
pub(super) struct DgmAttr {
    pub(super) id: ReeInt,
    pub(super) stackable: ReeInt,
    #[serde(rename = "highIsGood")]
    pub(super) high_is_good: ReeInt,
    #[serde(rename = "defaultValue")]
    pub(super) default: ReeFloat,
}
impl Into<dh::DgmAttr> for DgmAttr {
    fn into(self) -> dh::DgmAttr {
        dh::DgmAttr::new(self.id, self.stackable != 0, self.high_is_good != 0, self.default)
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct DgmEffect {
    pub(super) id: ReeInt,
    #[serde(rename = "effectCategory")]
    pub(super) category_id: ReeInt,
    #[serde(rename = "isAssistance")]
    pub(super) is_assistance: ReeInt,
    #[serde(rename = "isOffensive")]
    pub(super) is_offensive: ReeInt,
    #[serde(rename = "isWarpSafe")]
    pub(super) is_warp_safe: ReeInt,
    #[serde(rename = "dischargeAttributeID")]
    pub(super) discharge_attr_id: Option<ReeInt>,
    #[serde(rename = "durationAttributeID")]
    pub(super) duration_attr_id: Option<ReeInt>,
    #[serde(rename = "rangeAttributeID")]
    pub(super) range_attr_id: Option<ReeInt>,
    #[serde(rename = "falloffAttributeID")]
    pub(super) falloff_attr_id: Option<ReeInt>,
    #[serde(rename = "trackingSpeedAttributeID")]
    pub(super) tracking_attr_id: Option<ReeInt>,
    #[serde(rename = "fittingUsageChanceAttributeID")]
    pub(super) usage_chance_attr_id: Option<ReeInt>,
    #[serde(rename = "resistanceAttributeID")]
    pub(super) resist_attr_id: Option<ReeInt>,
    #[serde(rename = "modifierInfo", deserialize_with = "deserialize_dgmeffectmods", default)]
    pub(super) mods: Vec<DgmEffectMod>,
}
impl Into<dh::DgmEffect> for DgmEffect {
    fn into(self) -> dh::DgmEffect {
        dh::DgmEffect::new(
            self.id,
            self.category_id,
            self.is_assistance != 0,
            self.is_offensive != 0,
            self.is_warp_safe != 0,
            into_opt(self.discharge_attr_id),
            into_opt(self.duration_attr_id),
            into_opt(self.range_attr_id),
            into_opt(self.falloff_attr_id),
            into_opt(self.tracking_attr_id),
            into_opt(self.usage_chance_attr_id),
            into_opt(self.resist_attr_id),
            self.mods.into_iter().map(|v| v.into()).collect(),
        )
    }
}
#[derive(Debug)]
pub(super) struct DgmEffectMod {
    pub(super) func: String,
    pub(super) args: HashMap<String, dh::Primitive>,
}
impl Into<dh::DgmEffectMod> for DgmEffectMod {
    fn into(self) -> dh::DgmEffectMod {
        dh::DgmEffectMod::new(self.func, self.args)
    }
}
fn deserialize_dgmeffectmods<'de, D>(json_mods: D) -> result::Result<Vec<DgmEffectMod>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let func_field = "func";
    let mut mods = Vec::new();
    for json_mod in <Vec<serde_json::Value>>::deserialize(json_mods)?.into_iter() {
        let mut json_mod_map =
            <serde_json::Map<String, serde_json::Value>>::deserialize(json_mod).map_err(DeError::custom)?;
        // Process function name
        let func = match json_mod_map.remove(func_field) {
            Some(v) => v,
            None => return Err(DeError::missing_field(func_field)),
        };
        let func = match func {
            serde_json::Value::String(s) => s.to_owned(),
            _ => return Err(DeError::custom(format!("unexpected type of {} value", func_field))),
        };
        // Process arguments
        let mut argmap = HashMap::new();
        for (argname, v) in json_mod_map.into_iter() {
            let argval = match v {
                serde_json::Value::Null => dh::Primitive::Null,
                serde_json::Value::Bool(b) => dh::Primitive::Bool(b),
                serde_json::Value::Number(n) => {
                    if n.is_i64() {
                        dh::Primitive::Int(n.as_i64().unwrap() as ReeInt)
                    } else if n.is_f64() {
                        dh::Primitive::Float(n.as_f64().unwrap())
                    } else {
                        return Err(DeError::custom("unexpected number type"));
                    }
                }
                serde_json::Value::String(s) => dh::Primitive::String(s),
                _ => {
                    return Err(DeError::custom(format!(
                        "unexpected type of argument \"{}\" value",
                        argname
                    )))
                }
            };
            argmap.insert(argname, argval);
        }
        mods.push(DgmEffectMod { func, args: argmap })
    }
    Ok(mods)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Buffs
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize)]
pub(super) struct Buff {
    pub(super) id: ReeInt,
    #[serde(rename = "aggregateMode")]
    pub(super) aggregate: String,
    #[serde(rename = "operationName")]
    pub(super) operation: String,
    #[serde(rename = "itemModifiers")]
    pub(super) item_mods: Vec<BuffItemMod>,
    #[serde(rename = "locationModifiers")]
    pub(super) loc_mods: Vec<BuffLocMod>,
    #[serde(rename = "locationGroupModifiers")]
    pub(super) locgroup_mods: Vec<BuffLocGroupMod>,
    #[serde(rename = "locationRequiredSkillModifiers")]
    pub(super) locsrq_mods: Vec<BuffLocSrqMod>,
}
impl Into<dh::Buff> for Buff {
    fn into(self) -> dh::Buff {
        dh::Buff::new(
            self.id,
            self.aggregate,
            self.operation,
            into_vec(self.item_mods),
            into_vec(self.loc_mods),
            into_vec(self.locgroup_mods),
            into_vec(self.locsrq_mods),
        )
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct BuffItemMod {
    #[serde(rename = "dogmaAttributeID")]
    pub(super) attr_id: ReeInt,
}
impl Into<dh::BuffItemMod> for BuffItemMod {
    fn into(self) -> dh::BuffItemMod {
        dh::BuffItemMod::new(self.attr_id)
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct BuffLocMod {
    #[serde(rename = "dogmaAttributeID")]
    pub(super) attr_id: ReeInt,
}
impl Into<dh::BuffLocMod> for BuffLocMod {
    fn into(self) -> dh::BuffLocMod {
        dh::BuffLocMod::new(self.attr_id)
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct BuffLocGroupMod {
    #[serde(rename = "dogmaAttributeID")]
    pub(super) attr_id: ReeInt,
    #[serde(rename = "groupID")]
    pub(super) group_id: ReeInt,
}
impl Into<dh::BuffLocGroupMod> for BuffLocGroupMod {
    fn into(self) -> dh::BuffLocGroupMod {
        dh::BuffLocGroupMod::new(self.attr_id, self.group_id)
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct BuffLocSrqMod {
    #[serde(rename = "dogmaAttributeID")]
    pub(super) attr_id: ReeInt,
    #[serde(rename = "skillID")]
    pub(super) skill_id: ReeInt,
}
impl Into<dh::BuffLocSrqMod> for BuffLocSrqMod {
    fn into(self) -> dh::BuffLocSrqMod {
        dh::BuffLocSrqMod::new(self.attr_id, self.skill_id)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fighter abilities
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize)]
pub(super) struct FighterAbil {
    pub(super) id: ReeInt,
    #[serde(rename = "targetMode")]
    pub(super) target_mode: String,
    #[serde(rename = "disallowInHighSec")]
    pub(super) disallow_hisec: bool,
    #[serde(rename = "disallowInLowSec")]
    pub(super) disallow_lowsec: bool,
}
impl Into<dh::FighterAbil> for FighterAbil {
    fn into(self) -> dh::FighterAbil {
        dh::FighterAbil::new(self.id, &self.target_mode, self.disallow_hisec, self.disallow_lowsec)
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct TypeFighterAbil {
    pub(super) type_id: ReeInt,
    #[serde(rename = "abilitySlot0")]
    pub(super) abil0: Option<AbilExtras>,
    #[serde(rename = "abilitySlot1")]
    pub(super) abil1: Option<AbilExtras>,
    #[serde(rename = "abilitySlot2")]
    pub(super) abil2: Option<AbilExtras>,
}
impl Into<dh::TypeFighterAbil> for TypeFighterAbil {
    fn into(self) -> dh::TypeFighterAbil {
        dh::TypeFighterAbil::new(
            self.type_id,
            into_opt(self.abil0),
            into_opt(self.abil1),
            into_opt(self.abil2),
        )
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct AbilExtras {
    #[serde(rename = "abilityID")]
    pub(super) ability_id: ReeInt,
    #[serde(rename = "cooldownSeconds")]
    pub(super) cooldown: Option<ReeFloat>,
    pub(super) charges: Option<AbilChargeExtras>,
}
impl Into<dh::AbilExtras> for AbilExtras {
    fn into(self) -> dh::AbilExtras {
        dh::AbilExtras::new(self.ability_id, self.cooldown, into_opt(self.charges))
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct AbilChargeExtras {
    #[serde(rename = "chargeCount")]
    pub(super) count: ReeInt,
    #[serde(rename = "rearmTimeSeconds")]
    pub(super) rearm_time: ReeFloat,
}
impl Into<dh::AbilChargeExtras> for AbilChargeExtras {
    fn into(self) -> dh::AbilChargeExtras {
        dh::AbilChargeExtras::new(self.count, self.rearm_time)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize)]
pub(super) struct Metadata {
    pub(super) field_name: String,
    pub(super) field_value: u32,
}
