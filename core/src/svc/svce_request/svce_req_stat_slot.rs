use crate::{
    def::FitKey,
    svc::{Svc, SvcCtx, vast::StatSlot},
    uad::{Uad, UadFit},
};

impl Svc {
    pub(crate) fn get_stat_fit_high_slots(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_high_slots(SvcCtx::new(uad, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_mid_slots(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_mid_slots(SvcCtx::new(uad, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_low_slots(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_low_slots(SvcCtx::new(uad, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_turret_slots(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_turret_slots(SvcCtx::new(uad, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_launcher_slots(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_launcher_slots(SvcCtx::new(uad, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_rig_slots(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_rig_slots(SvcCtx::new(uad, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_service_slots(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_service_slots(SvcCtx::new(uad, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_subsystem_slots(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_subsystem_slots(SvcCtx::new(uad, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_launched_drones(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_launched_drones(SvcCtx::new(uad, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_launched_fighters(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_launched_fighters(SvcCtx::new(uad, &self.eprojs), &mut self.calc, fit)
    }
    pub(crate) fn get_stat_fit_launched_light_fighters(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        fit: &UadFit,
    ) -> StatSlot {
        self.vast.get_fit_data(&fit_key).get_stat_launched_light_fighters(
            SvcCtx::new(uad, &self.eprojs),
            &mut self.calc,
            fit,
        )
    }
    pub(crate) fn get_stat_fit_launched_heavy_fighters(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        fit: &UadFit,
    ) -> StatSlot {
        self.vast.get_fit_data(&fit_key).get_stat_launched_heavy_fighters(
            SvcCtx::new(uad, &self.eprojs),
            &mut self.calc,
            fit,
        )
    }
    pub(crate) fn get_stat_fit_launched_support_fighters(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        fit: &UadFit,
    ) -> StatSlot {
        self.vast.get_fit_data(&fit_key).get_stat_launched_support_fighters(
            SvcCtx::new(uad, &self.eprojs),
            &mut self.calc,
            fit,
        )
    }
    pub(crate) fn get_stat_fit_launched_st_light_fighters(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        fit: &UadFit,
    ) -> StatSlot {
        self.vast.get_fit_data(&fit_key).get_stat_launched_st_light_fighters(
            SvcCtx::new(uad, &self.eprojs),
            &mut self.calc,
            fit,
        )
    }
    pub(crate) fn get_stat_fit_launched_st_heavy_fighters(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        fit: &UadFit,
    ) -> StatSlot {
        self.vast.get_fit_data(&fit_key).get_stat_launched_st_heavy_fighters(
            SvcCtx::new(uad, &self.eprojs),
            &mut self.calc,
            fit,
        )
    }
    pub(crate) fn get_stat_fit_launched_st_support_fighters(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        fit: &UadFit,
    ) -> StatSlot {
        self.vast.get_fit_data(&fit_key).get_stat_launched_st_support_fighters(
            SvcCtx::new(uad, &self.eprojs),
            &mut self.calc,
            fit,
        )
    }
}
