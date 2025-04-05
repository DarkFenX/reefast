from dataclasses import dataclass


@dataclass
class ValOverloadSkillFail:

    td_lvl: int | None
    module_reqs: dict[str, int]

    def __init__(self, *, data: tuple) -> None:
        self.td_lvl = data[0]
        self.module_reqs = data[1]

    def __eq__(self, other: tuple) -> bool:
        return (self.td_lvl, self.module_reqs) == (other[0], other[1])
