use crate::{
    ss::{
        svc::debug::{check_effect, check_item},
        SsView,
    },
    util::DebugResult,
};

use super::RunningEffects;

impl RunningEffects {
    pub(in crate::ss::svc) fn debug_consistency_check(&self, ss_view: &SsView) -> DebugResult {
        for (item_id, effect_ids) in self.data.iter() {
            check_item(ss_view, item_id)?;
            for effect_id in effect_ids.iter() {
                check_effect(ss_view, effect_id)?;
            }
        }
        Ok(())
    }
}
