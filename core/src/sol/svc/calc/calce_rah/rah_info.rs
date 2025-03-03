use crate::{
    defs::AttrVal,
    sol::{SolDmgKinds, svc::calc::SolAttrVal},
};

// Initial values of a RAH, non-rounded
#[derive(Copy, Clone)]
pub(super) struct SolRahInfo {
    pub(super) resos: SolDmgKinds<SolAttrVal>,
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
            resos: SolDmgKinds::new(res_em, res_therm, res_kin, res_expl),
            cycle_time,
            shift_amount,
        }
    }
}
