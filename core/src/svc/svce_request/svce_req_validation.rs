use crate::{
    svc::{
        Svc, SvcCtx,
        vast::{ValOptionsInt, ValOptionsSolInt, ValResultFit, ValResultSol},
    },
    ud::{UData, UFitKey},
};

impl Svc {
    pub(crate) fn validate_sol_fast(&mut self, u_data: &UData, options: &ValOptionsSolInt) -> bool {
        self.vast
            .validate_sol_fast(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, options)
    }
    pub(crate) fn validate_sol_verbose(&mut self, u_data: &UData, options: &ValOptionsSolInt) -> ValResultSol {
        self.vast
            .validate_sol_verbose(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, options)
    }
    pub(crate) fn validate_fit_fast(&mut self, u_data: &UData, fit_key: UFitKey, options: &ValOptionsInt) -> bool {
        self.vast
            .validate_fit_fast(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, fit_key, options)
    }
    pub(crate) fn validate_fit_verbose(
        &mut self,
        u_data: &UData,
        fit_key: UFitKey,
        options: &ValOptionsInt,
    ) -> ValResultFit {
        self.vast
            .validate_fit_verbose(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, fit_key, options)
    }
}
