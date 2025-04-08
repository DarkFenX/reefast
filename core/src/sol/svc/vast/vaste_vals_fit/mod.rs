pub use vaste_activation_blocked::ValActivationBlockedFail;
pub use vaste_capital_module::ValCapitalModFail;
pub(in crate::sol::svc::vast) use vaste_charge_group::ValChargeGroupFailCache;
pub use vaste_charge_group::{ValChargeGroupChargeInfo, ValChargeGroupFail};
pub(in crate::sol::svc::vast) use vaste_charge_size::ValChargeSizeFailCache;
pub use vaste_charge_size::{ValChargeSizeChargeInfo, ValChargeSizeFail};
pub(in crate::sol::svc::vast) use vaste_charge_volume::ValChargeVolumeFailCache;
pub use vaste_charge_volume::{ValChargeVolumeChargeInfo, ValChargeVolumeFail};
pub use vaste_drone_group::ValDroneGroupFail;
pub use vaste_fighter_squad_size::{ValFighterSquadSizeFail, ValFighterSquadSizeFighterInfo};
pub use vaste_item_kind::{ValItemKindFail, ValItemKindItemInfo};
pub use vaste_item_vs_ship_kind::{ValItemVsShipKindFail, ValShipKind};
pub use vaste_max_group::{ValMaxGroupFail, ValMaxGroupGroupInfo};
pub use vaste_max_type::{ValMaxTypeFail, ValMaxTypeTypeInfo};
pub use vaste_module_state::{ValModuleStateFail, ValModuleStateModuleInfo};
pub use vaste_not_loaded_item::ValNotLoadedItemFail;
pub use vaste_overload_skill::ValOverloadSkillFail;
pub use vaste_resource::ValResFail;
pub use vaste_rig_size::ValRigSizeFail;
pub use vaste_sec_zone::ValSecZoneFail;
pub use vaste_ship_limit::{ValShipLimitFail, ValShipLimitItemInfo};
pub use vaste_ship_stance::ValShipStanceFail;
pub use vaste_skill_reqs::{ValSrqFail, ValSrqSkillInfo};
pub use vaste_slot_count::ValSlotCountFail;
pub use vaste_slot_index::ValSlotIndexFail;
pub use vaste_unusable_resource::ValUnusableResFail;
pub use vaste_unusable_slot::ValUnusableSlotFail;

mod shared;
mod vaste_activation_blocked;
mod vaste_capital_module;
mod vaste_charge_group;
mod vaste_charge_size;
mod vaste_charge_volume;
mod vaste_drone_group;
mod vaste_fighter_squad_size;
mod vaste_item_kind;
mod vaste_item_vs_ship_kind;
mod vaste_max_group;
mod vaste_max_type;
mod vaste_module_state;
mod vaste_not_loaded_item;
mod vaste_overload_skill;
mod vaste_resource;
mod vaste_rig_size;
mod vaste_sec_zone;
mod vaste_ship_limit;
mod vaste_ship_stance;
mod vaste_skill_reqs;
mod vaste_slot_count;
mod vaste_slot_index;
mod vaste_unusable_resource;
mod vaste_unusable_slot;
