pub(crate) use ordered_float::OrderedFloat as OF;

/// Full version of the library as a string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Entity IDs
pub type AbilId = i32;
pub type AttrId = i32;
pub type ItemId = u32;
pub type ItemTypeId = i32;
pub type ItemGrpId = i32;
pub type FitId = u32;
pub type FleetId = u32;
pub type DogmaEffectId = i32;
pub type CustomEffectId = i32;
// Misc
pub type AttrVal = OF<f64>;
pub type Count = u32;
pub type Idx = usize;
pub type SlotIndex = i32;
