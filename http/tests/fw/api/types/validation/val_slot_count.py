import dataclasses


@dataclasses.dataclass
class ValSlotCountFail:

    used: int
    max: int | None
    users: list[str]

    def __init__(self, *, data: tuple) -> None:
        self.used = data[0]
        self.max = data[1]
        self.users = sorted(data[2])
