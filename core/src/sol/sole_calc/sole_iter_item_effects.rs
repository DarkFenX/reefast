use crate::{
    err::basic::{ItemFoundError, ItemLoadedError},
    sol::{EffectId, EffectInfo, ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn iter_item_effects(
        &self,
        item_id: &ItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.iter_item_effects_internal(item_key)?)
    }
    pub(in crate::sol) fn iter_item_effects_internal(
        &self,
        item_key: ItemKey,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, ItemLoadedError> {
        let item = self.uad.items.get(item_key);
        let a_effect_ids = match item.get_a_effect_datas() {
            Some(a_effect_datas) => a_effect_datas.keys(),
            None => {
                return Err(ItemLoadedError {
                    item_id: item.get_item_id(),
                });
            }
        };
        let effect_infos = a_effect_ids.map(move |a_effect_id| {
            let running = self.svc.is_effect_running(&item_key, a_effect_id);
            let mode = *item.get_effect_modes().get(a_effect_id);
            (a_effect_id.into(), EffectInfo { running, mode })
        });
        Ok(effect_infos)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum IterItemEffectsError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
}
