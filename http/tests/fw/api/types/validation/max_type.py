from dataclasses import dataclass


class ValMaxTypeFail(dict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({int(k): ValMaxTypeType(data=v) for k, v in data.items()})


@dataclass
class ValMaxTypeType:

    item_type_count: int
    items: dict[str, int]

    def __init__(self, *, data: tuple) -> None:
        self.item_type_count = data[0]
        self.items = dict(data[1])

    def __eq__(self, other: tuple) -> bool:
        return (self.item_type_count, self.items) == (other[0], other[1])
