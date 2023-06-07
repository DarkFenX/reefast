use crate::{
    cmd::{CmdResp, FitCommand, ItemIdsResp, SsCommand},
    info::{HFitInfo, HFitInfoMode, HItemInfo, HItemInfoMode, HSsInfo, HSsInfoMode},
    util::{Error, ErrorKind, Result},
};

pub(crate) struct HSolarSystem {
    id: String,
    accessed: chrono::DateTime<chrono::Utc>,
    core_ss: Option<rc::SolarSystem>,
}
impl HSolarSystem {
    pub(crate) fn new(id: String, core_ss: rc::SolarSystem) -> Self {
        Self {
            id,
            accessed: chrono::Utc::now(),
            core_ss: Some(core_ss),
        }
    }
    pub(crate) fn last_accessed(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.accessed
    }
    pub(crate) async fn get_info(
        &mut self,
        ss_mode: HSsInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Result<HSsInfo> {
        let mut core_ss = self.take_ss()?;
        let ss_id_mv = self.id.clone();
        let (res, core_ss) = tokio_rayon::spawn_fifo(move || {
            let res = HSsInfo::mk_info(ss_id_mv, &mut core_ss, ss_mode, fit_mode, item_mode);
            (res, core_ss)
        })
        .await;
        self.core_ss = Some(core_ss);
        self.touch();
        Ok(res)
    }
    // Fit methods
    pub(crate) async fn add_fit(&mut self, fit_mode: HFitInfoMode, item_mode: HItemInfoMode) -> Result<HFitInfo> {
        let mut core_ss = self.take_ss()?;
        let (res, core_ss) = tokio_rayon::spawn_fifo(move || {
            let res = match core_ss.add_fit() {
                Ok(fit_id) => Ok(HFitInfo::mk_info(&mut core_ss, &fit_id, fit_mode, item_mode)),
                Err(e) => Err(e.into()),
            };
            (res, core_ss)
        })
        .await;
        self.core_ss = Some(core_ss);
        self.touch();
        res
    }
    pub(crate) async fn get_fit(
        &mut self,
        fit_id: &str,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Result<HFitInfo> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_ss = self.take_ss()?;
        let (res, core_ss) = tokio_rayon::spawn_fifo(move || {
            let res = HFitInfo::mk_info(&mut core_ss, &fit_id, fit_mode, item_mode);
            (res, core_ss)
        })
        .await;
        self.core_ss = Some(core_ss);
        self.touch();
        Ok(res)
    }
    pub(crate) async fn remove_fit(&mut self, fit_id: &str) -> Result<()> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_ss = self.take_ss()?;
        let (res, core_ss) = tokio_rayon::spawn_fifo(move || {
            let res = core_ss.remove_fit(&fit_id).map_err(|e| e.into());
            (res, core_ss)
        })
        .await;
        self.core_ss = Some(core_ss);
        self.touch();
        res
    }
    // Item methods
    pub(crate) async fn get_item(&mut self, item_id: &str, item_mode: HItemInfoMode) -> Result<HItemInfo> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_ss = self.take_ss()?;
        let (res, core_ss) = tokio_rayon::spawn_fifo(move || match core_ss.get_item_info(&item_id) {
            Ok(core_info) => {
                let item_info = HItemInfo::mk_info(&mut core_ss, &core_info, item_mode);
                (Ok(item_info), core_ss)
            }
            Err(e) => (Err(Error::from(e)), core_ss),
        })
        .await;
        self.core_ss = Some(core_ss);
        self.touch();
        res
    }
    pub(crate) async fn remove_item(&mut self, item_id: &str) -> Result<()> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_ss = self.take_ss()?;
        let (res, core_ss) = tokio_rayon::spawn_fifo(move || {
            let res = core_ss.remove_item(&item_id).map_err(|v| v.into());
            (res, core_ss)
        })
        .await;
        self.core_ss = Some(core_ss);
        self.touch();
        res
    }
    // Command methods
    pub(crate) async fn execute_ss_commands(
        &mut self,
        commands: Vec<SsCommand>,
        ss_mode: HSsInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Result<(HSsInfo, Vec<CmdResp>)> {
        let mut core_ss = self.take_ss()?;
        let ss_id_mv = self.id.clone();
        let (core_ss, ss_info, cmd_results) = tokio_rayon::spawn_fifo(move || {
            let cmd_results = execute_commands(&mut core_ss, commands);
            let info = HSsInfo::mk_info(ss_id_mv, &mut core_ss, ss_mode, fit_mode, item_mode);
            (core_ss, info, cmd_results)
        })
        .await;
        self.core_ss = Some(core_ss);
        self.touch();
        Ok((ss_info, cmd_results))
    }
    pub(crate) async fn execute_fit_commands(
        &mut self,
        fit_id: &str,
        commands: Vec<FitCommand>,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Result<(HFitInfo, Vec<CmdResp>)> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_ss = self.take_ss()?;
        let (core_ss, fit_info, cmd_results) = tokio_rayon::spawn_fifo(move || {
            let commands = commands.into_iter().map(|v| v.fill_fit(fit_id)).collect();
            let cmd_results = execute_commands(&mut core_ss, commands);
            let info = HFitInfo::mk_info(&mut core_ss, &fit_id, fit_mode, item_mode);
            (core_ss, info, cmd_results)
        })
        .await;
        self.core_ss = Some(core_ss);
        self.touch();
        Ok((fit_info, cmd_results))
    }
    // Helper methods
    fn take_ss(&mut self) -> Result<rc::SolarSystem> {
        match self.core_ss.take() {
            Some(core_ss) => Ok(core_ss),
            None => {
                self.touch();
                Err(Error::new(ErrorKind::NoCoreSs))
            }
        }
    }
    fn str_to_fit_id(&mut self, id: &str) -> Result<rc::ReeId> {
        match id.parse() {
            Ok(i) => Ok(i),
            Err(_) => {
                self.touch();
                Err(Error::new(ErrorKind::FitIdCastFailed(id.to_string())))
            }
        }
    }
    fn str_to_item_id(&mut self, id: &str) -> Result<rc::ReeId> {
        match id.parse() {
            Ok(i) => Ok(i),
            Err(_) => {
                self.touch();
                Err(Error::new(ErrorKind::ItemIdCastFailed(id.to_string())))
            }
        }
    }
    fn touch(&mut self) {
        self.accessed = chrono::Utc::now();
    }
}

fn execute_commands(core_ss: &mut rc::SolarSystem, commands: Vec<SsCommand>) -> Vec<CmdResp> {
    let mut cmd_results = Vec::with_capacity(commands.len());
    for cmd in commands.iter() {
        match cmd {
            SsCommand::AddImplant(c) => {
                let implant_id = core_ss
                    .add_implant(c.fit_id, c.type_id, c.state.unwrap_or(true))
                    .unwrap();
                let resp = CmdResp::ItemIds(ItemIdsResp::from(implant_id));
                cmd_results.push(resp);
            }
            SsCommand::SetShip(c) => {
                let ship_id = core_ss
                    .set_fit_ship(c.fit_id, c.type_id, c.state.unwrap_or(true))
                    .unwrap();
                let resp = CmdResp::ItemIds(ItemIdsResp::from(ship_id));
                cmd_results.push(resp);
            }
            SsCommand::AddModuleHigh(c) => {
                let id_data = core_ss
                    .add_module(
                        c.fit_id,
                        c.module_type_id,
                        c.state.into(),
                        rc::ModRack::High,
                        c.add_mode.into(),
                        c.charge_type_id,
                    )
                    .unwrap();
                let resp = CmdResp::ItemIds(ItemIdsResp::from(id_data));
                cmd_results.push(resp);
            }
            SsCommand::AddModuleMid(c) => {
                let id_data = core_ss
                    .add_module(
                        c.fit_id,
                        c.module_type_id,
                        c.state.into(),
                        rc::ModRack::Mid,
                        c.add_mode.into(),
                        c.charge_type_id,
                    )
                    .unwrap();
                let resp = CmdResp::ItemIds(ItemIdsResp::from(id_data));
                cmd_results.push(resp);
            }
            SsCommand::AddModuleLow(c) => {
                let id_data = core_ss
                    .add_module(
                        c.fit_id,
                        c.module_type_id,
                        c.state.into(),
                        rc::ModRack::Low,
                        c.add_mode.into(),
                        c.charge_type_id,
                    )
                    .unwrap();
                let resp = CmdResp::ItemIds(ItemIdsResp::from(id_data));
                cmd_results.push(resp);
            }
            SsCommand::AddRig(c) => {
                let rig_id = core_ss.add_rig(c.fit_id, c.type_id, c.state.unwrap_or(true)).unwrap();
                let resp = CmdResp::ItemIds(ItemIdsResp::from(rig_id));
                cmd_results.push(resp);
            }
        };
    }
    cmd_results
}
