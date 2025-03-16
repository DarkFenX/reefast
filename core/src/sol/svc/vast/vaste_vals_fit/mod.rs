pub use vaste_capital_module::{SolValCapitalModFail, SolValCapitalModItemInfo};
pub use vaste_charge_group::SolValChargeGroupFail;
pub use vaste_charge_size::SolValChargeSizeFail;
pub use vaste_charge_volume::SolValChargeVolumeFail;
pub use vaste_drone_group::{SolValDroneGroupFail, SolValDroneGroupItemInfo};
pub use vaste_fighter_count::SolValFighterCountFail;
pub use vaste_item_kind::SolValItemKindFail;
pub use vaste_max_group::{SolValMaxGroupFail, SolValMaxGroupItemInfo};
pub use vaste_module_state::SolValModuleStateFail;
pub use vaste_not_loaded_item::SolValNotLoadedItemFail;
pub use vaste_resource::{SolValResFail, SolValResItemInfo};
pub use vaste_rig_size::{SolValRigSizeFail, SolValRigSizeItemInfo};
pub use vaste_ship_limit::{SolValShipLimitFail, SolValShipLimitItemInfo};
pub use vaste_ship_stance::SolValShipStanceFail;
pub use vaste_skill_reqs::{SolValSrqFail, SolValSrqSkillInfo};
pub use vaste_slot_count::SolValSlotCountFail;
pub use vaste_slot_index::SolValSlotIndexFail;
pub use vaste_unusable_resource::{SolValUnusableResFail, SolValUnusableResItemInfo};
pub use vaste_unusable_slot::SolValUnusableSlotFail;

mod shared;
mod vaste_capital_module;
mod vaste_charge_group;
mod vaste_charge_size;
mod vaste_charge_volume;
mod vaste_drone_group;
mod vaste_fighter_count;
mod vaste_item_kind;
mod vaste_max_group;
mod vaste_module_state;
mod vaste_not_loaded_item;
mod vaste_resource;
mod vaste_rig_size;
mod vaste_ship_limit;
mod vaste_ship_stance;
mod vaste_skill_reqs;
mod vaste_slot_count;
mod vaste_slot_index;
mod vaste_unusable_resource;
mod vaste_unusable_slot;
