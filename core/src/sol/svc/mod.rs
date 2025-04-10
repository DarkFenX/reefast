//! Solar system services - attribute & stats calculations, validation, and so on.

pub(in crate::sol::svc) use misc::{AttrSpec, EffectSpec};
pub(in crate::sol) use svc::Svc;

pub(crate) mod calc;
mod misc;
mod running_effects;
mod svc;
mod svce_debug;
mod svce_effect;
mod svce_interface;
mod svce_notify;
pub(crate) mod vast;
