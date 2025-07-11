use crate::{
    cmd::{
        shared::get_primary_item,
        stats::options::{HStatOption, HStatOptionEhp, HStatResolvedOption},
    },
    info::HItemStats,
    util::HExecError,
};

#[derive(educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(crate) struct HGetItemStatsCmd {
    #[educe(Default = true)]
    default: bool,
    agility: Option<bool>,
    align_time: Option<bool>,
    speed: Option<bool>,
    hp: Option<bool>,
    ehp: Option<HStatOption<HStatOptionEhp>>,
    wc_ehp: Option<bool>,
    resists: Option<bool>,
}
impl HGetItemStatsCmd {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemStats, HExecError> {
        let mut core_item = get_primary_item(core_sol, item_id)?;
        let mut stats = HItemStats::new();
        if self.agility.unwrap_or(self.default) {
            stats.agility = core_item.get_stat_agility().into();
        }
        if self.align_time.unwrap_or(self.default) {
            stats.align_time = core_item.get_stat_align_time().into();
        }
        if self.speed.unwrap_or(self.default) {
            stats.speed = core_item.get_stat_speed().into();
        }
        if self.hp.unwrap_or(self.default) {
            stats.hp = core_item.get_stat_hp().into();
        }
        let ehp_opt = HStatResolvedOption::new(&self.ehp, self.default);
        if ehp_opt.enabled {
            stats.ehp = Some(
                ehp_opt
                    .options
                    .iter()
                    .map(|inner_opt| {
                        let core_incoming_dps = inner_opt.incoming_dps.map(|h_incoming_dps| h_incoming_dps.into());
                        core_item
                            .get_stat_ehp(core_incoming_dps.as_ref())
                            .map(|core_ehp| core_ehp.into())
                    })
                    .collect(),
            )
        }
        if self.wc_ehp.unwrap_or(self.default) {
            stats.wc_ehp = core_item.get_stat_wc_ehp().into();
        }
        if self.resists.unwrap_or(self.default) {
            stats.resists = core_item.get_stat_resists().into();
        }
        Ok(stats)
    }
}
