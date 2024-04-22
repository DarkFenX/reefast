use crate::{ss::SsView, util::DebugResult};

use super::SsSvcCalcData;

impl SsSvcCalcData {
    pub(in crate::ss::svc) fn debug_consistency_check(&self, ss_view: &SsView) -> DebugResult {
        self.attrs.debug_consistency_check(ss_view)?;
        self.mods.debug_consistency_check(ss_view)?;
        self.affectee.debug_consistency_check(ss_view)?;
        self.buffs.debug_consistency_check(ss_view)?;
        self.deps.debug_consistency_check(ss_view)?;
        self.revs.debug_consistency_check(ss_view)?;
        Ok(())
    }
}
