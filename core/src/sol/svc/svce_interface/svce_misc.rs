use crate::{
    ad,
    sol::{
        ItemId,
        svc::Svc,
        uad::{
            Uad,
            item::{Fighter, Ship, Skill},
        },
    },
    src::Src,
};

impl Svc {
    pub(in crate::sol) fn src_changed(&mut self, src: &Src) {
        self.notify_src_changed(src);
    }
    pub(in crate::sol) fn item_base_attr_value_changed(
        &mut self,
        uad: &Uad,
        item_id: &ItemId,
        a_attr_id: &ad::AAttrId,
    ) {
        self.notify_base_attr_value_changed(uad, item_id, a_attr_id);
    }
    pub(in crate::sol) fn sol_sec_zone_changed(&mut self, uad: &Uad) {
        self.notify_sol_sec_zone_changed(uad);
    }
    pub(in crate::sol) fn fighter_count_changed(&mut self, uad: &Uad, fighter: &Fighter) {
        self.notify_fighter_count_changed(uad, fighter);
    }
    pub(in crate::sol) fn ship_sec_status_changed(&mut self, uad: &Uad, ship: &Ship) {
        self.notify_ship_sec_status_changed(uad, ship);
    }
    pub(in crate::sol) fn skill_level_changed(&mut self, uad: &Uad, skill: &Skill) {
        self.notify_skill_level_changed(uad, skill);
    }
}
