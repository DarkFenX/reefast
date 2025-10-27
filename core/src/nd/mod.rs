//! ND stands for eNtity Data.
//!
//! This module is a place for almost all the hardcoded customizations applied to various EVE
//! entities, and entities derived from them.

pub(crate) use eff::{
    N_EFFECT_MAP, N_EFFECTS, NBreacherDmgGetter, NCalcCustomizer, NDmgKindGetter, NEcmGetter, NEffect, NEffectCharge,
    NEffectChargeDepl, NEffectChargeLoc, NEffectDmgKind, NEffectHc, NLocalRepGetter, NNeutGetter, NNormalDmgGetter,
    NProjMultGetter, NRemoteRepGetter, NSpoolResolver,
};

mod eff;
