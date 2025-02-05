//! Solar system user & adapted data.

pub use item::{
    SolItemAddAttrMutation, SolItemAddMutation, SolItemAttrMutationValue, SolItemChangeAttrMutation, SolMinionState,
    SolModuleState,
};
pub(in crate::sol) use uad::SolUad;

pub(in crate::sol) mod fit;
pub(in crate::sol) mod fleet;
pub(in crate::sol) mod item;
mod uad;
mod uade_debug;
