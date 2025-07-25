import dataclasses

from tests.fw.util import Absent
from .opt_shared import StatOptionAlias, StatOptionEhpAlias, StatOptionErpsAlias, StatOptionRpsAlias, dc_to_dict

type StatOptionFitDpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionFitDps]]
type StatOptionFitVolleyAlias = StatOptionAlias | tuple[bool, list[StatsOptionFitVolley]]
type StatOptionFitRemoteRpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionFitRemoteRps]]


@dataclasses.dataclass(kw_only=True)
class FitStatsOptions:

    default: bool = False
    high_slots: StatOptionAlias = Absent
    mid_slots: StatOptionAlias = Absent
    low_slots: StatOptionAlias = Absent
    turret_slots: StatOptionAlias = Absent
    launcher_slots: StatOptionAlias = Absent
    rig_slots: StatOptionAlias = Absent
    service_slots: StatOptionAlias = Absent
    subsystem_slots: StatOptionAlias = Absent
    launched_drones: StatOptionAlias = Absent
    launched_fighters: StatOptionAlias = Absent
    launched_light_fighters: StatOptionAlias = Absent
    launched_heavy_fighters: StatOptionAlias = Absent
    launched_support_fighters: StatOptionAlias = Absent
    launched_st_light_fighters: StatOptionAlias = Absent
    launched_st_heavy_fighters: StatOptionAlias = Absent
    launched_st_support_fighters: StatOptionAlias = Absent
    cpu: StatOptionAlias = Absent
    powergrid: StatOptionAlias = Absent
    calibration: StatOptionAlias = Absent
    drone_bay_volume: StatOptionAlias = Absent
    drone_bandwidth: StatOptionAlias = Absent
    fighter_bay_volume: StatOptionAlias = Absent
    agility: StatOptionAlias = Absent
    align_time: StatOptionAlias = Absent
    speed: StatOptionAlias = Absent
    dps: StatOptionFitDpsAlias = Absent
    volley: StatOptionFitVolleyAlias = Absent
    hp: StatOptionAlias = Absent
    ehp: StatOptionEhpAlias = Absent
    wc_ehp: StatOptionAlias = Absent
    rps: StatOptionRpsAlias = Absent
    erps: StatOptionErpsAlias = Absent
    resists: StatOptionAlias = Absent
    remote_rps: StatOptionFitRemoteRpsAlias = Absent
    remote_cps: StatOptionAlias = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitDps:

    reload: bool | type[Absent] = Absent
    spool: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitVolley:

    spool: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitRemoteRps:

    spool: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
