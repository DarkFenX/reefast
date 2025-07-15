//! Various helper entities used throughout the library.

pub(crate) use funcs::{ceil_unerr, floor_unerr, round, sig_round, trunc_unerr, vec_push_opt};
pub(crate) use inf_count::InfCount;
pub use storage::{Map, RMap, RSet};
pub(crate) use storage::{MapSet, RMapRMap, RMapRSet, RMapVec, extend_vec_from_map_set_l1};
pub(crate) use str_err::StrMsgError;
pub(crate) use traits::{GetId, Named};
pub use unit_interval::{UnitInterval, UnitIntervalError};

mod funcs;
mod inf_count;
mod storage;
mod str_err;
mod traits;
mod unit_interval;
