use std::collections::HashSet;

use crate::{
    defs::{EEffectId, SsItemId},
    util::KeyedStorage1L,
};

pub(in crate::ss::svc) struct RunningEffects {
    data: KeyedStorage1L<SsItemId, EEffectId>,
}
impl RunningEffects {
    pub(in crate::ss::svc) fn new() -> Self {
        Self {
            data: KeyedStorage1L::new(),
        }
    }
    // Getters
    pub(in crate::ss::svc) fn get_running_effects(&self, item_id: &SsItemId) -> Option<&HashSet<EEffectId>> {
        self.data.get(item_id)
    }
    // Maintenance methods
    pub(in crate::ss::svc) fn effects_started<I>(&mut self, item_id: SsItemId, effects: I)
    where
        I: Iterator<Item = EEffectId> + ExactSizeIterator,
    {
        self.data.extend(item_id, effects);
    }
    pub(in crate::ss::svc) fn effects_stopped<I>(&mut self, item_id: &SsItemId, effects: I)
    where
        I: Iterator<Item = EEffectId>,
    {
        self.data.drain(item_id, effects);
    }
}
