use super::Projs;
use crate::sol::{
    debug::{DebugResult, check_item_key},
    uad::Uad,
};

impl Projs {
    pub(in crate::sol::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for &projectee_item_key in self.iter_projectee_item_keys() {
            check_item_key(uad, projectee_item_key, false)?;
        }
        Ok(())
    }
}
