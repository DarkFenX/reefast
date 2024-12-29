use crate::{
    defs::AttrVal,
    sol::{svc::svce_calc::SolAttrVal, SolDmgTypes},
};

#[derive(Copy, Clone)]
pub(super) struct SolRahInfo {
    pub(super) resos: SolDmgTypes<SolAttrVal>,
    pub(super) cycle_time: AttrVal,
    pub(super) shift_amount: AttrVal,
}
impl SolRahInfo {
    pub(super) fn new(
        res_em: SolAttrVal,
        res_therm: SolAttrVal,
        res_kin: SolAttrVal,
        res_expl: SolAttrVal,
        cycle_time: AttrVal,
        shift_amount: AttrVal,
    ) -> Self {
        Self {
            resos: SolDmgTypes::new(res_em, res_therm, res_kin, res_expl),
            cycle_time,
            shift_amount,
        }
    }
}
