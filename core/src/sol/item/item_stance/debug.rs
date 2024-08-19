use crate::sol::{item::debug, SolDebugResult, SolView};

use super::SolStance;

impl SolStance {
    pub(in crate::sol::item) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        debug::check_fit(sol_view, &self.get_fit_id())?;
        Ok(())
    }
}
