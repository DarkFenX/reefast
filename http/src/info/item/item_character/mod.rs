use full::HCharacterInfoFull;
use id::HCharacterInfoId;
use partial::HCharacterInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HCharacterInfo {
    Id(HCharacterInfoId),
    Partial(HCharacterInfoPartial),
    Full(HCharacterInfoFull),
}
impl HCharacterInfo {
    pub(crate) fn mk_info(core_character: &mut rc::CharacterMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_character.into()),
            HItemInfoMode::Partial => Self::Partial(core_character.into()),
            HItemInfoMode::Full => Self::Full(core_character.into()),
        }
    }
}
