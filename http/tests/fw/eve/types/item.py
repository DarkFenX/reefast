from __future__ import annotations

import typing

from tests.fw.util import Absent, conditional_insert, make_repr_str
from .exception import TestDataConsistencyError

if typing.TYPE_CHECKING:
    from tests.fw.eve.containers.primitives import EvePrimitives


class Item:

    def __init__(
            self, *,
            id_: int,
            group_id: int | type[Absent],
            attributes: dict[int, float] | type[Absent],
            effect_ids: list[int] | type[Absent],
            default_effect_id: int | None,
            skill_reqs: dict[int, int] | type[Absent],
            capacity: float | type[Absent],
            mass: float | type[Absent],
            radius: float | type[Absent],
            volume: float | type[Absent],
    ) -> None:
        self.id = id_
        self.group_id = group_id
        self.attributes = attributes
        self.effect_ids = effect_ids
        self.default_effect_id = default_effect_id
        self.skill_reqs = skill_reqs
        self.capacity = capacity
        self.mass = mass
        self.radius = radius
        self.volume = volume

    def to_primitives(self, *, primitive_data: EvePrimitives) -> None:
        item_entry = {'typeID': self.id}
        conditional_insert(container=item_entry, key='groupID', value=self.group_id)
        self.__add_primitive_item_attributes(primitive_data=primitive_data)
        self.__add_primitive_item_effects(primitive_data=primitive_data)
        conditional_insert(container=primitive_data.requiredskillsfortypes, key=self.id, value=self.skill_reqs)
        conditional_insert(container=item_entry, key='capacity', value=self.capacity)
        conditional_insert(container=item_entry, key='mass', value=self.mass)
        conditional_insert(container=item_entry, key='radius', value=self.radius)
        conditional_insert(container=item_entry, key='volume', value=self.volume)
        if self.id in primitive_data.types:
            msg = f'attempt to add item with duplicate ID {self.id}'
            raise TestDataConsistencyError(msg)
        primitive_data.types[self.id] = item_entry

    def __add_primitive_item_attributes(self, *, primitive_data: EvePrimitives) -> None:
        if self.attributes is Absent:
            return
        item_entry = primitive_data.typedogma.setdefault(self.id, {})
        if isinstance(self.attributes, dict):
            attrs_entry = item_entry['dogmaAttributes'] = []
            for attr_id, attr_val in self.attributes.items():
                attrs_entry.append({'attributeID': attr_id, 'value': attr_val})
        else:
            item_entry['dogmaAttributes'] = self.attributes

    def __add_primitive_item_effects(self, *, primitive_data: EvePrimitives) -> None:
        if self.effect_ids is Absent:
            return
        item_entry = primitive_data.typedogma.setdefault(self.id, {})
        if isinstance(self.effect_ids, list):
            item_entry['dogmaEffects'] = [
                {'effectID': e, 'isDefault': int(e == self.default_effect_id)}
                for e in self.effect_ids]
        else:
            item_entry['dogmaEffects'] = self.effect_ids

    def __repr__(self) -> str:
        return make_repr_str(instance=self)
