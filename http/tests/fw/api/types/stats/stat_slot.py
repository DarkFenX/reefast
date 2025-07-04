
import dataclasses
import typing


@dataclasses.dataclass
class StatSlot:

    used: int
    total: int | None

    def __init__(self, *, data: tuple) -> None:
        self.used = data[0]
        self.total = data[1]

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.used, self.total) == (other[0], other[1])
