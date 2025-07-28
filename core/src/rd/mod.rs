//! RD stands for Runtime Data.
//!
//! This module contains all the entities enriched during runtime. Runtime modifications serve 3
//! distinct purposes:
//! - Precalculating and exposing some of an entity attributes in immediately available way to save
//!   resources when they are needed. Some of the data could've been calculated on cache generation
//!   and persisted, but it makes cache handler more complex;
//! - Combining adapted and hardcoded data (as well as derived from both of those) under one roof;
//! - Remapping some IDs to slab keys for faster access for some entities.

pub use data::RItemKind;
pub(crate) use data::{
    RAbil, RAttr, RBuff, RData, REffect, REffectKey, RItem, RItemAXt, RItemChargeLimit, RItemContLimit, RItemShipLimit,
    RMuta, RShipDroneLimit, RShipKind, RcAbil, RcAttr, RcBuff, RcEffect, RcItem, RcMuta,
};

mod data;
