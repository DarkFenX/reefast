//! Adapted data generator

use crate::{
    ad,
    adg::{eff_abil::get_abil_effect, support::GSupport},
    ed,
    src::SrcInitError,
};

mod eff_abil;
mod flow;
mod rels;
mod support;

/// Fetch EVE data and generate adapted data out of it
#[tracing::instrument(name = "adg", level = "trace", skip_all)]
pub(crate) fn generate_adapted_data(e_handler: &dyn ed::EveDataHandler) -> Result<ad::AData, SrcInitError> {
    let mut g_supp = GSupport::new();
    let mut a_data = ad::AData::new();
    let mut e_data = flow::fetch_data(e_handler).map_err(|e| SrcInitError::EveDataFetchFailed(e.to_string()))?;
    flow::dedup_pks(&mut e_data);
    flow::normalize(&mut e_data);
    g_supp.fill(&e_data);
    flow::clean_unused(&mut e_data, &g_supp).map_err(|e| SrcInitError::EveDataCleanupFailed(e.to_string()))?;
    flow::validate(&mut e_data, &g_supp);
    flow::convert(&e_data, &g_supp, &mut a_data);
    flow::customize(&mut a_data);
    flow::fill_extra_data(&mut a_data, &g_supp);
    Ok(a_data)
}
