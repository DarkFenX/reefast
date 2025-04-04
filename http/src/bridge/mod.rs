// This module contains entities which bridge synchronous core lib and async HTTP interface
// together.

pub(crate) use err_res::HBrError;
pub(crate) use sol::HGuardedSol;
pub(crate) use sol_mgr::HSolMgr;
pub(crate) use src_mgr::HSrcMgr;

mod err_res;
mod sol;
mod sol_mgr;
mod src_mgr;
