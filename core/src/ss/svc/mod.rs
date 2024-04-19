//! Solar system services - attribute & stats calculations, restrictions, and so on.

pub(in crate::ss::svc) use misc::debug;
pub(in crate::ss) use svc::SsSvcs;
pub use svce_calc::{SsAttrVal, SsModInfo, SsModOpInfo, SsModSrcInfo, SsModSrcValInfo};

mod misc;
mod svc;
mod svce_calc;
mod svce_debug;
mod svce_routing;
