pub use add::AddRangedProjError;
pub use get::GetRangedProjError;
pub use iter::RangedProjIter;
pub(in crate::sol::api) use iter::iter_ranged_projs;
pub use ranged_proj::{RangedProj, RangedProjMut};

mod add;
mod get;
mod iter;
mod ranged_proj;
mod remove;
mod set_range;
