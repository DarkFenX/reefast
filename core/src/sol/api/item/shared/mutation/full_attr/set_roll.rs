use crate::{misc::AttrMutationRequest, sol::api::FullMAttrMut, util::UnitInterval};

impl<'a> FullMAttrMut<'a> {
    /// Set roll for the attribute.
    ///
    /// None as value removes user-defined mutation.
    pub fn set_roll(&mut self, roll: Option<UnitInterval>) {
        let attr_mutation_request = vec![AttrMutationRequest {
            attr_id: self.a_attr_id,
            value: roll,
        }];
        self.sol
            .internal_change_item_mutation_attrs(self.item_key, attr_mutation_request)
            .unwrap();
    }
}
