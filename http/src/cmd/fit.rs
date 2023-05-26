use crate::cmd::shared::{AddMode, State};

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum FitCommand {
    SetShip(SetShipCmd),
    AddModuleHigh(AddModule),
    AddModuleMid(AddModule),
    AddModuleLow(AddModule),
}

#[derive(serde::Deserialize)]
pub(crate) struct SetShipCmd {
    pub(crate) ship_type_id: reefast::ReeInt,
}

#[derive(serde::Deserialize)]
pub(crate) struct AddModule {
    pub(crate) add_mode: AddMode,
    pub(crate) module_type_id: reefast::ReeInt,
    pub(crate) state: State,
    pub(crate) charge_type_id: Option<reefast::ReeInt>,
}
