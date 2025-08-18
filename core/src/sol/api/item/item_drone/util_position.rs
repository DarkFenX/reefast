use crate::{
    sol::{SolarSystem, rev_projs::RevProjs},
    svc::Svc,
    ud::{UData, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_update_drone_position(
        u_data: &mut UData,
        rev_projs: &RevProjs,
        svc: &mut Svc,
        drone_key: UItemKey,
    ) {
        let u_drone = u_data.items.get_mut(drone_key).get_drone_mut().unwrap();
        let u_position = *u_drone.get_position();
        // Handle outgoing projections
        if !u_drone.get_projs_mut().is_empty() {
            for u_proj_data in u_drone.get_projs_mut().iter_datas_mut() {
                u_proj_data.update_src_pos(u_position);
            }
            let u_drone = u_data.items.get(drone_key).get_drone().unwrap();
            for (projectee_key, u_proj_data) in u_drone.get_projs().iter_projectees_and_datas() {
                SolarSystem::util_change_item_proj_data(u_data, svc, drone_key, projectee_key, Some(u_proj_data));
            }
        }
        // Handle incoming projections
        SolarSystem::util_update_position_for_incoming(u_data, rev_projs, svc, drone_key, u_position);
    }
}
