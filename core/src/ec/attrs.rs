#![allow(dead_code)]

use crate::defs::EAttrId;

pub(crate) const SKILL_LEVEL: EAttrId = 280;
pub(crate) const SPEED: EAttrId = 51;
pub(crate) const ROF_BONUS: EAttrId = 293;
pub(crate) const DMG_MULT_BONUS: EAttrId = 292;
pub(crate) const DMG_MULT: EAttrId = 64;
// Damage attrs
pub(crate) const EM_DMG: EAttrId = 114;
pub(crate) const THERM_DMG: EAttrId = 118;
pub(crate) const KIN_DMG: EAttrId = 117;
pub(crate) const EXPL_DMG: EAttrId = 116;
// Fitting resource-related
pub(crate) const CPU: EAttrId = 50;
pub(crate) const CPU_OUTPUT: EAttrId = 48;
pub(crate) const POWER: EAttrId = 30;
pub(crate) const POWER_OUTPUT: EAttrId = 11;
// Slot-related
pub(crate) const BOOSTERNESS: EAttrId = 1087;
pub(crate) const IMPLANTNESS: EAttrId = 331;
pub(crate) const SUBSYSTEM_SLOT: EAttrId = 1366;
// Fighter-related
pub(crate) const FTR_SQ_IS_HEAVY: EAttrId = 2214;
pub(crate) const FTR_SQ_IS_LIGHT: EAttrId = 2212;
pub(crate) const FTR_SQ_IS_SUPPORT: EAttrId = 2213;
// Buff-related
pub(crate) const WARFARE_BUFF1_ID: EAttrId = 2468;
pub(crate) const WARFARE_BUFF2_ID: EAttrId = 2470;
pub(crate) const WARFARE_BUFF3_ID: EAttrId = 2472;
pub(crate) const WARFARE_BUFF4_ID: EAttrId = 2536;

pub(crate) const BUFF_ID_ATTRS: [EAttrId; 4] = [WARFARE_BUFF1_ID, WARFARE_BUFF2_ID, WARFARE_BUFF3_ID, WARFARE_BUFF4_ID];
