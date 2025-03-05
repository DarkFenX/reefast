#![allow(dead_code)]

use crate::defs::EAttrId;

pub(crate) const MASS: EAttrId = 4;
pub(crate) const POWER_OUTPUT: EAttrId = 11;
pub(crate) const LOW_SLOTS: EAttrId = 12;
pub(crate) const MED_SLOTS: EAttrId = 13;
pub(crate) const HI_SLOTS: EAttrId = 14;
pub(crate) const SPEED_FACTOR: EAttrId = 20;
pub(crate) const POWER: EAttrId = 30;
pub(crate) const MAX_VELOCITY: EAttrId = 37;
pub(crate) const CAPACITY: EAttrId = 38;
pub(crate) const CPU_OUTPUT: EAttrId = 48;
pub(crate) const CPU: EAttrId = 50;
pub(crate) const SPEED: EAttrId = 51;
pub(crate) const DMG_MULT: EAttrId = 64;
pub(crate) const ARMOR_DMG_AMOUNT: EAttrId = 84;
pub(crate) const LAUNCHER_SLOTS_LEFT: EAttrId = 101;
pub(crate) const TURRET_SLOTS_LEFT: EAttrId = 102;
pub(crate) const WARP_SCRAMBLE_STATUS: EAttrId = 104;
pub(crate) const WARP_SCRAMBLE_STRENGTH: EAttrId = 105;
pub(crate) const EM_DMG: EAttrId = 114;
pub(crate) const EXPL_DMG: EAttrId = 116;
pub(crate) const KIN_DMG: EAttrId = 117;
pub(crate) const THERM_DMG: EAttrId = 118;
pub(crate) const CHARGE_SIZE: EAttrId = 128;
pub(crate) const VOLUME: EAttrId = 161;
pub(crate) const MISSILE_DMG_MULT: EAttrId = 212;
pub(crate) const ARMOR_EM_DMG_RESONANCE: EAttrId = 267;
pub(crate) const ARMOR_EXPL_DMG_RESONANCE: EAttrId = 268;
pub(crate) const ARMOR_KIN_DMG_RESONANCE: EAttrId = 269;
pub(crate) const ARMOR_THERM_DMG_RESONANCE: EAttrId = 270;
pub(crate) const SKILL_LEVEL: EAttrId = 280;
pub(crate) const DRONE_CAPACITY: EAttrId = 283;
pub(crate) const DMG_MULT_BONUS: EAttrId = 292;
pub(crate) const ROF_BONUS: EAttrId = 293;
pub(crate) const IMPLANTNESS: EAttrId = 331;
pub(crate) const MAX_ACTIVE_DRONES: EAttrId = 352;
pub(crate) const SIG_RADIUS: EAttrId = 552;
pub(crate) const SIG_RADIUS_BONUS: EAttrId = 554;
pub(crate) const SPEED_BOOST_FACTOR: EAttrId = 567;
pub(crate) const CHARGE_GROUP1: EAttrId = 604;
pub(crate) const CHARGE_GROUP2: EAttrId = 605;
pub(crate) const CHARGE_GROUP3: EAttrId = 606;
pub(crate) const CHARGE_GROUP4: EAttrId = 609;
pub(crate) const CHARGE_GROUP5: EAttrId = 610;
pub(crate) const MAX_GROUP_ACTIVE: EAttrId = 763;
pub(crate) const MASS_ADDITION: EAttrId = 796;
pub(crate) const DISALLOW_ASSISTANCE: EAttrId = 854;
pub(crate) const MAX_GROUP_ONLINE: EAttrId = 978;
pub(crate) const BOOSTERNESS: EAttrId = 1087;
pub(crate) const UPGRADE_CAPACITY: EAttrId = 1132;
pub(crate) const UPGRADE_COST: EAttrId = 1153;
pub(crate) const UPGRADE_SLOTS_LEFT: EAttrId = 1154;
pub(crate) const DRONE_BANDWIDTH: EAttrId = 1271;
pub(crate) const DRONE_BANDWIDTH_USED: EAttrId = 1272;
pub(crate) const CAN_FIT_SHIP_GROUP1: EAttrId = 1298;
pub(crate) const CAN_FIT_SHIP_GROUP2: EAttrId = 1299;
pub(crate) const CAN_FIT_SHIP_GROUP3: EAttrId = 1300;
pub(crate) const CAN_FIT_SHIP_GROUP4: EAttrId = 1301;
pub(crate) const CAN_FIT_SHIP_TYPE1: EAttrId = 1302;
pub(crate) const CAN_FIT_SHIP_TYPE2: EAttrId = 1303;
pub(crate) const CAN_FIT_SHIP_TYPE3: EAttrId = 1304;
pub(crate) const CAN_FIT_SHIP_TYPE4: EAttrId = 1305;
pub(crate) const MAX_RANGE_HIDDEN: EAttrId = 1317;
pub(crate) const ACTIVATION_BLOCKED: EAttrId = 1349;
pub(crate) const ACTIVATION_BLOCKED_STRENGTH: EAttrId = 1350;
pub(crate) const SUBSYSTEM_SLOT: EAttrId = 1366;
pub(crate) const MAX_SUBSYSTEMS: EAttrId = 1367;
pub(crate) const TURRET_HARDPOINT_MODIFIER: EAttrId = 1368;
pub(crate) const LAUNCHER_HARDPOINT_MODIFIER: EAttrId = 1369;
pub(crate) const HI_SLOT_MODIFIER: EAttrId = 1374;
pub(crate) const MED_SLOT_MODIFIER: EAttrId = 1375;
pub(crate) const LOW_SLOT_MODIFIER: EAttrId = 1376;
pub(crate) const FITS_TO_SHIP_TYPE: EAttrId = 1380;
pub(crate) const MAX_GROUP_FITTED: EAttrId = 1544;
pub(crate) const RIG_SIZE: EAttrId = 1547;
pub(crate) const ALLOWED_DRONE_GROUP1: EAttrId = 1782;
pub(crate) const ALLOWED_DRONE_GROUP2: EAttrId = 1783;
pub(crate) const RESIST_SHIFT_AMOUNT: EAttrId = 1849;
pub(crate) const CAN_FIT_SHIP_GROUP5: EAttrId = 1872;
pub(crate) const CAN_FIT_SHIP_GROUP6: EAttrId = 1879;
pub(crate) const CAN_FIT_SHIP_GROUP7: EAttrId = 1880;
pub(crate) const CAN_FIT_SHIP_GROUP8: EAttrId = 1881;
pub(crate) const CHARGED_ARMOR_DMG_MULT: EAttrId = 1886;
pub(crate) const CAN_FIT_SHIP_TYPE5: EAttrId = 1944;
pub(crate) const GATE_SCRAMBLE_STATUS: EAttrId = 1973;
pub(crate) const GATE_SCRAMBLE_STRENGTH: EAttrId = 1974;
pub(crate) const CAN_FIT_SHIP_GROUP9: EAttrId = 2065;
pub(crate) const CAN_FIT_SHIP_TYPE6: EAttrId = 2103;
pub(crate) const REMOTE_RESISTANCE_ID: EAttrId = 2138;
pub(crate) const FTR_SQ_IS_LIGHT: EAttrId = 2212;
pub(crate) const FTR_SQ_IS_SUPPORT: EAttrId = 2213;
pub(crate) const FTR_SQ_IS_HEAVY: EAttrId = 2214;
pub(crate) const FTR_TUBES: EAttrId = 2216;
pub(crate) const FTR_LIGHT_SLOTS: EAttrId = 2217;
pub(crate) const FTR_SUPPORT_SLOTS: EAttrId = 2218;
pub(crate) const FTR_HEAVY_SLOTS: EAttrId = 2219;
pub(crate) const SPEED_FACTOR_FLOOR: EAttrId = 2266;
pub(crate) const DOOMSDAY_AOE_RANGE: EAttrId = 2279;
pub(crate) const FTR_ABIL_BOMB_TYPE: EAttrId = 2324;
pub(crate) const CAN_FIT_SHIP_GROUP10: EAttrId = 2396;
pub(crate) const CAN_FIT_SHIP_TYPE7: EAttrId = 2463;
pub(crate) const WARFARE_BUFF1_ID: EAttrId = 2468;
pub(crate) const WARFARE_BUFF1_VAL: EAttrId = 2469;
pub(crate) const WARFARE_BUFF2_ID: EAttrId = 2470;
pub(crate) const WARFARE_BUFF2_VAL: EAttrId = 2471;
pub(crate) const WARFARE_BUFF3_ID: EAttrId = 2472;
pub(crate) const WARFARE_BUFF3_VAL: EAttrId = 2473;
pub(crate) const CAN_FIT_SHIP_GROUP11: EAttrId = 2476;
pub(crate) const CAN_FIT_SHIP_GROUP12: EAttrId = 2477;
pub(crate) const CAN_FIT_SHIP_GROUP13: EAttrId = 2478;
pub(crate) const CAN_FIT_SHIP_GROUP14: EAttrId = 2479;
pub(crate) const CAN_FIT_SHIP_GROUP15: EAttrId = 2480;
pub(crate) const CAN_FIT_SHIP_GROUP16: EAttrId = 2481;
pub(crate) const CAN_FIT_SHIP_GROUP17: EAttrId = 2482;
pub(crate) const CAN_FIT_SHIP_GROUP18: EAttrId = 2483;
pub(crate) const CAN_FIT_SHIP_GROUP19: EAttrId = 2484;
pub(crate) const CAN_FIT_SHIP_GROUP20: EAttrId = 2485;
pub(crate) const CAN_FIT_SHIP_TYPE8: EAttrId = 2486;
pub(crate) const CAN_FIT_SHIP_TYPE9: EAttrId = 2487;
pub(crate) const CAN_FIT_SHIP_TYPE10: EAttrId = 2488;
pub(crate) const WARFARE_BUFF4_ID: EAttrId = 2536;
pub(crate) const WARFARE_BUFF4_VAL: EAttrId = 2537;
pub(crate) const FTR_STANDUP_LIGHT_SLOTS: EAttrId = 2737;
pub(crate) const FTR_STANDUP_SUPPORT_SLOTS: EAttrId = 2738;
pub(crate) const FTR_STANDUP_HEAVY_SLOTS: EAttrId = 2739;
pub(crate) const FTR_SQ_IS_STANDUP_LIGHT: EAttrId = 2740;
pub(crate) const FTR_SQ_IS_STANDUP_SUPPORT: EAttrId = 2741;
pub(crate) const FTR_SQ_IS_STANDUP_HEAVY: EAttrId = 2742;
pub(crate) const CAN_FIT_SHIP_TYPE11: EAttrId = 2758;
