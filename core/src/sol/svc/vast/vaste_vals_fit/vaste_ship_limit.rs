use std::collections::HashMap;

use crate::{
    ad,
    sol::{ItemGrpId, ItemId, ItemTypeId, svc::vast::VastFitData, uad::item::Ship},
    util::RSet,
};

pub struct ValShipLimitFail {
    /// Type ID of current ship.
    pub ship_type_id: Option<ItemTypeId>,
    /// Group ID of current ship.
    pub ship_group_id: Option<ItemGrpId>,
    /// Map with IDs of items which cannot be fit to current ship, with their requirements.
    pub items: HashMap<ItemId, ValShipLimitItemInfo>,
}

pub struct ValShipLimitItemInfo {
    /// Ship type IDs item can be fit to.
    pub allowed_type_ids: Vec<ItemTypeId>,
    /// Ship group IDs item can be fit to.
    pub allowed_group_ids: Vec<ItemGrpId>,
}
impl From<&ad::AItemShipLimit> for ValShipLimitItemInfo {
    fn from(item_ship_limit: &ad::AItemShipLimit) -> Self {
        Self {
            allowed_type_ids: item_ship_limit.type_ids.clone(),
            allowed_group_ids: item_ship_limit.group_ids.clone(),
        }
    }
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_ship_limit_fast(&self, kfs: &RSet<ItemId>, ship: Option<&Ship>) -> bool {
        let ship = match ship {
            Some(ship) => ship,
            None => {
                return match kfs.is_empty() {
                    true => self.ship_limited_items.is_empty(),
                    false => self.ship_limited_items.difference(kfs).next().is_none(),
                };
            }
        };
        let ship_type_id = ship.get_a_item_id();
        let ship_group_id = ship.get_a_group_id();
        for (limited_item_id, ship_limit) in self.ship_limited_items.iter() {
            if ship_limit.type_ids.contains(&ship_type_id) {
                continue;
            }
            if let Some(ship_group_id) = ship_group_id {
                if ship_limit.group_ids.contains(&ship_group_id) {
                    continue;
                }
            }
            if kfs.contains(limited_item_id) {
                continue;
            }
            return false;
        }
        true
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_ship_limit_verbose(
        &self,
        kfs: &RSet<ItemId>,
        ship: Option<&Ship>,
    ) -> Option<ValShipLimitFail> {
        if self.ship_limited_items.is_empty() {
            return None;
        }
        let (ship_type_id, ship_group_id) = match ship {
            Some(ship) => (Some(ship.get_a_item_id()), ship.get_a_group_id()),
            None => (None, None),
        };
        let mut mismatches = HashMap::new();
        for (limited_item_id, ship_limit) in self.ship_limited_items.iter() {
            if let Some(ship_type_id) = ship_type_id {
                if ship_limit.type_ids.contains(&ship_type_id) {
                    continue;
                }
            }
            if let Some(ship_group_id) = ship_group_id {
                if ship_limit.group_ids.contains(&ship_group_id) {
                    continue;
                }
            }
            if kfs.contains(limited_item_id) {
                continue;
            }
            mismatches.insert(*limited_item_id, ship_limit.into());
        }
        match mismatches.is_empty() {
            true => None,
            false => Some(ValShipLimitFail {
                ship_type_id,
                ship_group_id,
                items: mismatches,
            }),
        }
    }
}
