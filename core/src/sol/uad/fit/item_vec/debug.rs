use super::ItemVec;
use crate::sol::debug::{DebugError, DebugResult};

impl ItemVec {
    pub(in crate::sol) fn consistency_check(&self) -> DebugResult {
        if self.data.iter().filter(|v| v.is_some()).count() != self.item_count {
            return Err(DebugError {});
        }
        Ok(())
    }
}
