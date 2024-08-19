use crate::sol::{item::debug, SolDebugResult, SolView};

use super::SolFighter;

impl SolFighter {
    pub(in crate::sol::item) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        debug::check_fit(sol_view, &self.get_fit_id())?;
        self.get_autocharges().debug_consistency_check(sol_view)?;
        self.get_projs().debug_consistency_check(sol_view)?;
        Ok(())
    }
}
