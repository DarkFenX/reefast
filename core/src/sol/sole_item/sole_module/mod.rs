pub use pos_modes::{AddMode, RmMode};
pub use sole_add_module::AddModuleError;
pub use sole_add_module_mutation::AddModuleMutationError;
pub use sole_add_module_proj::AddModuleProjError;
pub use sole_change_module_mutation::ChangeModuleMutationError;
pub use sole_change_module_proj::ChangeModuleProjError;
pub use sole_get_fit_modules::GetFitModulesError;
pub use sole_get_module::GetModuleError;
pub use sole_remove_module::RemoveModuleError;
pub use sole_remove_module_charge::RemoveModuleChargeError;
pub use sole_remove_module_mutation::RemoveModuleMutationError;
pub use sole_remove_module_proj::RemoveModuleProjError;
pub use sole_set_module_charge::SetModuleChargeError;
pub use sole_set_module_state::SetModuleStateError;

mod pos_modes;
mod shared;
mod sole_add_module;
mod sole_add_module_mutation;
mod sole_add_module_proj;
mod sole_change_module_mutation;
mod sole_change_module_proj;
mod sole_get_fit_modules;
mod sole_get_module;
mod sole_remove_module;
mod sole_remove_module_charge;
mod sole_remove_module_mutation;
mod sole_remove_module_proj;
mod sole_set_module_charge;
mod sole_set_module_state;
