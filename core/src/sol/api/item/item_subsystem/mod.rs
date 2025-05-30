pub use sol_get_subsystem::GetSubsystemError;
pub use subsystem::{Subsystem, SubsystemMut};

mod fit_add_subsystem;
mod fit_iter_subsystems;
mod sol_get_subsystem;
mod subsystem;
mod subsystem_remove;
mod subsystem_set_state;
mod subsystem_set_type_id;
mod util_add_remove;
