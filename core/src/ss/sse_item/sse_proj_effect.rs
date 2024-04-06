use crate::{
    defs::{EItemId, SsItemId},
    ss::{
        info::SsProjEffectInfo,
        item::{SsItem, SsProjEffect},
        SolarSystem, SsView,
    },
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_proj_effect_info(&self, item_id: &SsItemId) -> Result<SsProjEffectInfo> {
        Ok(self.items.get_proj_effect(item_id)?.into())
    }
    pub fn get_proj_effect_infos(&self) -> Vec<SsProjEffectInfo> {
        self.proj_effects
            .iter()
            .map(|v| self.items.get_proj_effect(v).unwrap().into())
            .collect()
    }
    pub fn add_proj_effect(&mut self, a_item_id: EItemId, state: bool) -> Result<SsProjEffectInfo> {
        let item_id = self.items.alloc_item_id()?;
        let proj_effect = SsProjEffect::new(&self.src, item_id, a_item_id, state);
        let info = SsProjEffectInfo::from(&proj_effect);
        let item = SsItem::ProjEffect(proj_effect);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_proj_effect_state(&mut self, item_id: &SsItemId, state: bool) -> Result<()> {
        let proj_effect = self.items.get_proj_effect_mut(item_id)?;
        let old_state = proj_effect.state;
        proj_effect.set_bool_state(state);
        let new_state = proj_effect.state;
        if new_state != old_state {
            let item = self.items.get_item(item_id).unwrap();
            self.svcs.switch_item_state(
                &SsView::new(&self.src, &self.fits, &self.items),
                item,
                old_state,
                new_state,
            );
        }
        Ok(())
    }
}
