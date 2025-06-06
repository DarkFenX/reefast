
import dataclasses
import typing


class ValItemKindFail(dict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValItemKindInfo(data=v) for k, v in data.items()})


@dataclasses.dataclass
class ValItemKindInfo:

    kind: str | None
    expected_kind: str

    def __init__(self, *, data: tuple) -> None:
        self.kind = data[0]
        self.expected_kind = data[1]

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.kind, self.expected_kind) == (other[0], other[1])
