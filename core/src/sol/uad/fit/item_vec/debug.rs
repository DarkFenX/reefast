use crate::sol::debug::{DebugError, DebugResult};

use super::ItemVec;

impl ItemVec {
    pub(in crate::sol) fn debug_consistency_check(&self) -> DebugResult {
        if self.data.iter().filter(|v| v.is_some()).count() != self.item_count {
            return Err(DebugError::new());
        }
        Ok(())
    }
}
