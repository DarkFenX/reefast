use full::HModuleInfoFull;
use id::HModuleInfoId;
use partial::HModuleInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HModuleInfo {
    Id(HModuleInfoId),
    Partial(HModuleInfoPartial),
    Full(HModuleInfoFull),
}
impl HModuleInfo {
    pub(crate) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_module_info: &rc::ModuleInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HModuleInfoId::mk_info(core_sol, core_module_info, item_mode)),
            HItemInfoMode::Partial => Self::Partial(HModuleInfoPartial::mk_info(core_sol, core_module_info, item_mode)),
            HItemInfoMode::Full => Self::Full(HModuleInfoFull::mk_info(core_sol, core_module_info, item_mode)),
        }
    }
}
