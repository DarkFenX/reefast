//! Cache generator.

use data::{Data, Support};

use crate::{dh::DataHandler, util::Result};

mod clean;
mod data;
mod fetch;
mod pk;

pub fn generate_cache(data_handler: &dyn DataHandler) -> Result<()> {
    let mut data = Data::new();
    let mut errs = Vec::new();
    let mut support = Support::new();
    fetch::fetch_data(data_handler, &mut data)?;
    pk::dedup_pks(&mut data, &mut errs);
    support.post_pk(&data);
    Ok(())
}
