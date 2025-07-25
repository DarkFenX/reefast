//! Solar system item definitions.

pub(crate) use base::{ItemMutationData, UEffectUpdates, get_combined_attr_values};
use base::{UItemBase, UItemBaseMutable};
pub(crate) use container::UItems;
pub(crate) use item::UItem;
pub(crate) use item_autocharge::UAutocharge;
pub(crate) use item_booster::UBooster;
pub(crate) use item_character::UCharacter;
pub(crate) use item_charge::UCharge;
pub(crate) use item_drone::UDrone;
pub(crate) use item_fighter::UFighter;
pub(crate) use item_fw_effect::UFwEffect;
pub(crate) use item_implant::UImplant;
pub(crate) use item_module::UModule;
pub(crate) use item_proj_effect::UProjEffect;
pub(crate) use item_rig::URig;
pub(crate) use item_service::UService;
pub(crate) use item_ship::{UShip, UShipKind};
pub(crate) use item_skill::USkill;
pub(crate) use item_stance::UStance;
pub(crate) use item_subsystem::USubsystem;
pub(crate) use item_sw_effect::USwEffect;
pub(crate) use misc::{Autocharges, Projs, UProjRange};
use misc::{bool_to_state_active, bool_to_state_offline, state_to_bool};

mod base;
mod container;
mod item;
mod item_autocharge;
mod item_booster;
mod item_character;
mod item_charge;
mod item_drone;
mod item_fighter;
mod item_fw_effect;
mod item_implant;
mod item_module;
mod item_proj_effect;
mod item_rig;
mod item_service;
mod item_ship;
mod item_skill;
mod item_stance;
mod item_subsystem;
mod item_sw_effect;
mod misc;
