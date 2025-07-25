pub(crate) use effect_charge::{NEffectCharge, NEffectChargeDepl, NEffectChargeLoc};

pub(in crate::nd::eff) mod damp;
mod effect_charge;
pub(in crate::nd::eff) mod missile_dmg_self_srq;
pub(in crate::nd::eff) mod opc_rep;
pub(in crate::nd::eff) mod proj_mult;
pub(in crate::nd::eff) mod prop_mods;
pub(in crate::nd::eff) mod sov_stability_generators;
pub(in crate::nd::eff) mod subsystem_mods;
pub(in crate::nd::eff) mod util;
pub(in crate::nd::eff) mod wd;
pub(in crate::nd::eff) mod web;
