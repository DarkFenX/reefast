use crate::{
    def::ItemKey,
    misc::ServiceState,
    sol::{
        SolarSystem,
        api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    },
    uad::UadService,
};

pub struct Service<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: ItemKey,
}
impl<'a> Service<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_state(&self) -> ServiceState {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for Service<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> ItemKey {
        self.key
    }
}
impl<'a> ItemCommon for Service<'a> {}

pub struct ServiceMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: ItemKey,
}
impl<'a> ServiceMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_uad_service(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_state(&self) -> ServiceState {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for ServiceMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> ItemKey {
        self.key
    }
}
impl<'a> ItemMutSealed for ServiceMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for ServiceMut<'a> {}
impl<'a> ItemMutCommon for ServiceMut<'a> {}

fn get_fit(sol: &SolarSystem, item_key: ItemKey) -> Fit<'_> {
    let fit_key = get_uad_service(sol, item_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_state(sol: &SolarSystem, item_key: ItemKey) -> ServiceState {
    get_uad_service(sol, item_key).get_service_state()
}
fn get_uad_service(sol: &SolarSystem, item_key: ItemKey) -> &UadService {
    sol.uad.items.get(item_key).get_service().unwrap()
}
