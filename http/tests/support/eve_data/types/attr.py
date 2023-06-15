from tests.support.util import conditional_insert, make_repr_str
from .exception import TestDataConsistencyError


class Attribute:

    def __init__(
            self,
            id_,
            stackable,
            high_is_good,
            default_value,
            max_attribute_id
    ):
        self.id = id_
        self.stackable = stackable
        self.high_is_good = high_is_good
        self.default_value = default_value
        self.max_attribute_id = max_attribute_id

    def to_primitives(self, primitive_data):
        attr_entry = {'attributeID': self.id}
        conditional_insert(attr_entry, 'stackable', self.stackable, cast_to=int)
        conditional_insert(attr_entry, 'highIsGood', self.high_is_good, cast_to=int)
        conditional_insert(attr_entry, 'defaultValue', self.default_value, cast_to=float)
        conditional_insert(attr_entry, 'maxAttributeID', self.max_attribute_id, cast_to=int)
        if self.id in primitive_data.dogmaattributes:
            raise TestDataConsistencyError(f'attempt to add attribute with duplicate ID {self.id}')
        primitive_data.dogmaattributes[self.id] = attr_entry

    def __repr__(self):
        return make_repr_str(self)
