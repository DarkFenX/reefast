
import dataclasses
import typing


class ValChargeGroupFail(dict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValChargeGroupInfo(data=v) for k, v in data.items()})


@dataclasses.dataclass
class ValChargeGroupInfo:

    parent_item_id: str
    charge_group_id: int
    allowed_group_ids: list[int]

    def __init__(self, *, data: tuple) -> None:
        self.parent_item_id = data[0]
        self.charge_group_id = data[1]
        self.allowed_group_ids = sorted(data[2])

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.parent_item_id, self.charge_group_id, self.allowed_group_ids) == (
            other[0], other[1], sorted(other[2]))
