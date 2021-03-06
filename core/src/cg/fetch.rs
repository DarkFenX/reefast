//! Contains facilities which fetch data from a data handler and store it in a generator-specific
//! container.

use log;

use crate::{
    dh::{self, DataHandler},
    util::{Error, Named, Result},
};

use super::data::Data;

const MAX_WARNS: usize = 5;

/// Fetch data from a data handler into a data vec, and report warnings, if any were encountered.
fn fetch_data_vec<S, F, T>(handler: &S, func: F, vec: &mut Vec<T>) -> Result<()>
where
    S: ?Sized + DataHandler,
    F: Fn(&S) -> dh::Result<dh::Container<T>>,
    T: Named,
{
    log::debug!("fetching {}", T::get_name());
    let cont = func(handler).map_err(|e| Error::new(format!("{}", e)))?;
    vec.extend(cont.data);
    let warn_amt = cont.warns.len();
    if warn_amt > 0 {
        log::warn!(
            "{} warnings encountered during fetching of {}, showing up to {}:",
            warn_amt,
            T::get_name(),
            MAX_WARNS
        );
        for warn_msg in cont.warns.iter().take(MAX_WARNS) {
            log::warn!("{}", warn_msg);
        }
    }
    Ok(())
}

pub(super) fn fetch_data(data_handler: &dyn DataHandler, data: &mut Data) -> Result<()> {
    log::debug!("using {:?} to fetch data", data_handler);
    fetch_data_vec(data_handler, DataHandler::get_items, &mut data.items)?;
    fetch_data_vec(data_handler, DataHandler::get_item_groups, &mut data.groups)?;
    fetch_data_vec(data_handler, DataHandler::get_attrs, &mut data.attrs)?;
    fetch_data_vec(data_handler, DataHandler::get_item_attrs, &mut data.item_attrs)?;
    fetch_data_vec(data_handler, DataHandler::get_effects, &mut data.effects)?;
    fetch_data_vec(data_handler, DataHandler::get_item_effects, &mut data.item_effects)?;
    fetch_data_vec(data_handler, DataHandler::get_fighter_abils, &mut data.abils)?;
    fetch_data_vec(data_handler, DataHandler::get_item_fighter_abils, &mut data.item_abils)?;
    fetch_data_vec(data_handler, DataHandler::get_buffs, &mut data.buffs)?;
    fetch_data_vec(data_handler, DataHandler::get_item_skill_reqs, &mut data.item_srqs)?;
    fetch_data_vec(data_handler, DataHandler::get_muta_item_convs, &mut data.muta_items)?;
    fetch_data_vec(data_handler, DataHandler::get_muta_attr_mods, &mut data.muta_attrs)?;
    Ok(())
}
