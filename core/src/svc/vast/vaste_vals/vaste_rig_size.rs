use std::collections::HashMap;

use crate::{
    ac, ad,
    def::{AttrVal, ItemId},
    svc::{SvcCtx, vast::VastFitData},
    ud::{UItemKey, UShip},
    util::RSet,
};

pub struct ValRigSizeFail {
    /// Rig size compatible with the ship.
    pub allowed_size: AttrVal,
    /// Sizes of incompatible rigs.
    pub rig_sizes: HashMap<ItemId, Option<AttrVal>>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_rig_size_fast(&self, kfs: &RSet<UItemKey>, ship: Option<&UShip>) -> bool {
        let allowed_size = match get_allowed_size(ship) {
            Some(allowed_size) => allowed_size,
            None => return true,
        };
        for (rig_key, &rig_size) in self.rigs_rig_size.iter() {
            if rig_size != Some(allowed_size) && !kfs.contains(rig_key) {
                return false;
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_rig_size_verbose(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        ship: Option<&UShip>,
    ) -> Option<ValRigSizeFail> {
        let allowed_size = get_allowed_size(ship)?;
        let mut rig_sizes = HashMap::new();
        for (rig_key, &rig_size) in self.rigs_rig_size.iter() {
            if rig_size != Some(allowed_size) && !kfs.contains(rig_key) {
                rig_sizes.insert(ctx.u_data.items.id_by_key(*rig_key), rig_size);
            }
        }
        match rig_sizes.is_empty() {
            true => None,
            false => Some(ValRigSizeFail {
                allowed_size,
                rig_sizes,
            }),
        }
    }
}

fn get_allowed_size(ship: Option<&UShip>) -> Option<ad::AAttrVal> {
    ship?.get_attrs()?.get(&ac::attrs::RIG_SIZE).copied()
}
