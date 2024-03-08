//! Solar system services - attribute & stats calculations, restrictions, and so on.

pub(in crate::ss) use svc::SsSvcs;
pub use svce_calc::SsAttrVal;

mod misc;
mod svc;
mod svce_calc;
mod svce_routing;
