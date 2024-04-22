use crate::{
    sol::{item::debug, SolView},
    util::DebugResult,
};

use super::SolModule;

impl SolModule {
    pub(in crate::sol::item) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        for tgt_item_id in self.tgts.iter_tgts() {
            debug::check_item(sol_view, tgt_item_id)?;
        }
        Ok(())
    }
}
