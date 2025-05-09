use crate::{
    ad,
    sol::{ItemKey, svc::vast::Vast, uad::item::UadItem},
};

impl Vast {
    pub(in crate::sol::svc) fn item_state_activated(
        &mut self,
        item_key: ItemKey,
        item: &UadItem,
        a_state: &ad::AState,
    ) {
    }
    pub(in crate::sol::svc) fn item_state_deactivated(
        &mut self,
        item_key: &ItemKey,
        item: &UadItem,
        a_state: &ad::AState,
    ) {
    }
}
