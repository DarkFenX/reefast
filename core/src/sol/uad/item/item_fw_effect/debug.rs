use crate::sol::{
    debug::{SolDebugResult, check_fit},
    uad::SolUad,
};

use super::SolFwEffect;

impl SolFwEffect {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        check_fit(uad, &self.get_fit_id())?;
        Ok(())
    }
}
