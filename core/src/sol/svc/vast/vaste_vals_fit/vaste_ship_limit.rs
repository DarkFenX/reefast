use crate::{
    ad,
    defs::{EItemGrpId, EItemId, SolItemId},
    sol::{svc::vast::SolVastFitData, uad::item::SolShip},
    util::StSet,
};

pub struct SolValShipLimitFail {
    pub ship_type_id: Option<EItemId>,
    pub ship_group_id: Option<EItemGrpId>,
    pub items: Vec<SolValShipLimitItemInfo>,
}

pub struct SolValShipLimitItemInfo {
    pub item_id: SolItemId,
    pub allowed_type_ids: Vec<EItemId>,
    pub allowed_group_ids: Vec<EItemGrpId>,
}
impl SolValShipLimitItemInfo {
    fn from_ship_limit(item_id: SolItemId, item_ship_limit: &ad::AItemShipLimit) -> Self {
        Self {
            item_id,
            allowed_type_ids: item_ship_limit.type_ids.clone(),
            allowed_group_ids: item_ship_limit.group_ids.clone(),
        }
    }
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_ship_limit_fast(
        &self,
        kfs: &StSet<SolItemId>,
        ship: Option<&SolShip>,
    ) -> bool {
        let ship = match ship {
            Some(ship) => ship,
            None => {
                return match kfs.is_empty() {
                    true => self.ship_limited_items.is_empty(),
                    false => self.ship_limited_items.difference(kfs).nth(0).is_none(),
                };
            }
        };
        let ship_type_id = ship.get_type_id();
        let ship_group_id = ship.get_group_id();
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
        kfs: &StSet<SolItemId>,
        ship: Option<&SolShip>,
    ) -> Option<SolValShipLimitFail> {
        if self.ship_limited_items.is_empty() {
            return None;
        }
        let (ship_type_id, ship_group_id) = match ship {
            Some(ship) => (Some(ship.get_type_id()), ship.get_group_id()),
            None => (None, None),
        };
        let mut mismatches = Vec::new();
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
            let mismatch = SolValShipLimitItemInfo::from_ship_limit(*limited_item_id, ship_limit);
            mismatches.push(mismatch);
        }
        match mismatches.is_empty() {
            true => None,
            false => Some(SolValShipLimitFail {
                ship_type_id,
                ship_group_id,
                items: mismatches,
            }),
        }
    }
}
