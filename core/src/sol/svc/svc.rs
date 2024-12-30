use crate::src::Src;

use super::{misc::SolRunningEffects, svce_calc::SolSvcCalcData};

// TODO: add item, remove item, add projection and remove projection methods are not called in
// situations where type ID of an item changes (e.g. item mutation / unmutation, source switch with
// mutation assigned). When there are users of underlying notification methods, should review them,
// and if there are any which rely on item type ID, should call those in situations where type ID
// can potentially change
#[derive(Clone)]
pub(in crate::sol) struct SolSvcs {
    pub(in crate::sol::svc) running_effects: SolRunningEffects,
    pub(in crate::sol::svc) calc_data: SolSvcCalcData,
}
impl SolSvcs {
    pub(in crate::sol) fn new(src: &Src) -> Self {
        Self {
            running_effects: SolRunningEffects::new(),
            calc_data: SolSvcCalcData::new(src),
        }
    }
}
