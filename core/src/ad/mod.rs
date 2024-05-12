//! Adapted data handler and data types.
//!
//! Adapted data types are built from the EVE data provided by an EVE data handler and are optimized
//! for ease of use by various reefast components. Adapted data types are assumed to be read-only by
//! the components, anything mutable is built on top of them.

pub use data::{
    AAttr, ABuff, ABuffAffecteeFilter, ABuffAggrMode, ABuffModifier, AData, AEffect, AEffectAffecteeFilter,
    AEffectBuffDataSrc, AEffectBuffInfo, AEffectBuffScope, AEffectDomain, AEffectModBuildStatus, AEffectModifier,
    AItem, AItemEffectData, AItemKind, AModifierSrq, AMuta, AMutaAttrRange, AOp, AState, ArcAttr, ArcBuff, ArcEffect,
    ArcItem, ArcMuta,
};
pub use handler::AdaptedDataHandler;
pub use result::AResult;

mod data;
mod handler;
mod result;
