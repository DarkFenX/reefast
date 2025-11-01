import dataclasses

from tests.fw.util import Absent, dc_to_dict


@dataclasses.dataclass(kw_only=True)
class StatCapSrcKinds:

    default: bool | type[Absent] = Absent
    regen: bool | StatCapRegenOptions | type[Absent] = Absent
    cap_boosters: bool | type[Absent] = Absent
    consumers: bool | type[Absent] = Absent
    incoming_transfers: bool | type[Absent] = Absent
    incoming_neuts: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatCapRegenOptions:

    enabled: bool | type[Absent] = Absent
    cap_perc: float | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
