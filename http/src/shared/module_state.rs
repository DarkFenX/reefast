#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HModuleState {
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}
impl From<&rc::ModuleState> for HModuleState {
    fn from(core_module_state: &rc::ModuleState) -> Self {
        match core_module_state {
            rc::ModuleState::Ghost => Self::Ghost,
            rc::ModuleState::Offline => Self::Offline,
            rc::ModuleState::Online => Self::Online,
            rc::ModuleState::Active => Self::Active,
            rc::ModuleState::Overload => Self::Overload,
        }
    }
}
impl From<&HModuleState> for rc::ModuleState {
    fn from(h_module_state: &HModuleState) -> Self {
        match h_module_state {
            HModuleState::Ghost => Self::Ghost,
            HModuleState::Offline => Self::Offline,
            HModuleState::Online => Self::Online,
            HModuleState::Active => Self::Active,
            HModuleState::Overload => Self::Overload,
        }
    }
}
