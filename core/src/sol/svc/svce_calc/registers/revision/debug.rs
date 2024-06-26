use crate::{
    sol::{svc::svce_calc::debug::check_ctx_modifier, SolView},
    util::DebugResult,
};

use super::SolRevisionRegister;

impl SolRevisionRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        for ctx_modifier in self.item_add.iter() {
            check_ctx_modifier(sol_view, ctx_modifier)?;
        }
        for ctx_modifier in self.item_remove.iter() {
            check_ctx_modifier(sol_view, ctx_modifier)?;
        }
        Ok(())
    }
}
