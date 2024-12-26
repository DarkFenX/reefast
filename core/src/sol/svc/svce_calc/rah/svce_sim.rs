use crate::{
    defs::SolFitId,
    sol::{svc::SolSvcs, SolView},
};

impl SolSvcs {
    pub(super) fn calc_rah_run_simulation(&mut self, sol_view: &SolView, fit_id: &SolFitId) {
        self.calc_data.rah.sim_running = true;
        self.calc_data.rah.sim_running = false;
    }
}
