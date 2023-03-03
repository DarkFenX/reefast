use std::collections::HashMap;

use crate::{consts::State, ss::item::Item, ReeFloat, ReeId, ReeInt};

use super::affection_reg::AffectionRegister;

pub(in crate::ss) struct CalcSvc {
    attrs: HashMap<ReeId, HashMap<ReeInt, ReeFloat>>,
    affection: AffectionRegister,
}
impl CalcSvc {
    pub(in crate::ss) fn new() -> CalcSvc {
        CalcSvc {
            attrs: HashMap::new(),
            affection: AffectionRegister::new(),
        }
    }
    // Query methods
    pub(in crate::ss) fn get_modifications(&mut self, afectee_item: &Item, afectee_attr_id: ReeInt) {}
    // Maintenance methods
    pub(in crate::ss) fn item_loaded(&mut self, item: &Item) {
        self.attrs.insert(item.get_id(), HashMap::new());
        self.affection.reg_afee(item);
    }
    pub(in crate::ss) fn item_unloaded(&mut self, item: &Item) {
        self.affection.unreg_afee(item);
        self.attrs.remove(&item.get_id());
    }
    pub(in crate::ss) fn effects_started(&mut self, item: &Item, state: State) {
        self.affection.activate_item_state(item, state);
    }
    pub(in crate::ss) fn effects_stopped(&mut self, item: &Item, state: State) {
        self.affection.deactivate_item_state(item, state);
    }
}
