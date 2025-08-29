//! Various helper entities used throughout the library.

pub(crate) use funcs::{
    ceil_tick, ceil_unerr, floor_tick, floor_unerr, round, round_unerr, sig_round, trunc_unerr, vec_push_opt,
};
pub(crate) use inf_count::InfCount;
pub use storage::{Map, RMap, RSet};
pub(crate) use storage::{MapSet, RMapRMap, RMapRMapRMap, RMapRSet, RMapVec, extend_vec_from_map_set_l1};
pub(crate) use str_err::StrMsgError;
pub(crate) use traits::{GetId, Named};
pub use unit_interval::{UnitInterval, UnitIntervalError};
pub(crate) use xyz::Xyz;

mod funcs;
mod inf_count;
mod storage;
mod str_err;
mod traits;
mod unit_interval;
mod xyz;
