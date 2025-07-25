use crate::{
    def::OF,
    misc::{DpsProfile, SecZone, Spool},
    src::Src,
    ud::{UItemKey, fit::UFits, fleet::UFleets, item::UItems},
    util::{RSet, UnitInterval},
};

// UAD stands for User and Adapted Data. Per definition, contains user-defined data, as well as some
// adapted data, stored on user-defined entities for optimization purposes.
#[derive(Clone)]
pub(crate) struct UData {
    pub(crate) src: Src,
    pub(crate) fleets: UFleets,
    pub(crate) fits: UFits,
    pub(crate) sw_effects: RSet<UItemKey>,
    pub(crate) proj_effects: RSet<UItemKey>,
    pub(crate) items: UItems,
    pub(crate) sec_zone: SecZone,
    pub(crate) default_spool: Spool,
    pub(crate) default_incoming_dps: DpsProfile,
}
impl UData {
    pub(crate) fn new(src: Src) -> Self {
        Self {
            src,
            fleets: UFleets::new(5),
            fits: UFits::new(50),
            sw_effects: RSet::new(),
            proj_effects: RSet::new(),
            items: UItems::new(10000),
            sec_zone: SecZone::NullSec,
            default_spool: Spool::SpoolScale(UnitInterval::new_clamped_of64(OF(1.0))),
            default_incoming_dps: DpsProfile::try_new(OF(1.0), OF(1.0), OF(1.0), OF(1.0), None).unwrap(),
        }
    }
}
