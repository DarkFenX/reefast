use super::UadShip;
use crate::sol::{
    debug::{DebugResult, check_fit_key},
    uad::Uad,
};

impl UadShip {
    pub(in crate::sol::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        Ok(())
    }
}
