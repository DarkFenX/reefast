use crate::sol::{
    item::{SolItem, SolItemState},
    svc::SolSvcs,
    SolView,
};

impl SolSvcs {
    pub(in crate::sol) fn switch_item_state(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        old_item_state: SolItemState,
        new_item_state: SolItemState,
    ) {
        if new_item_state > old_item_state {
            for state in SolItemState::iter().filter(|v| **v > old_item_state && **v <= new_item_state) {
                self.notify_state_activated(sol_view, item, state);
                if item.is_loaded() {
                    self.notify_item_state_activated_loaded(sol_view, item, state);
                }
            }
        } else if new_item_state < old_item_state {
            for state in SolItemState::iter().filter(|v| **v > new_item_state && **v <= old_item_state) {
                if item.is_loaded() {
                    self.notify_item_state_deactivated_loaded(sol_view, item, state);
                }
                self.notify_state_deactivated(sol_view, item, state);
            }
        }
        self.process_effects(sol_view, item, new_item_state);
    }
}
