import dataclasses
import typing

from tests.fw.util import Absent, dc_to_dict

if typing.TYPE_CHECKING:
    from .opt_shared import (
        StatsOptionCapBalance,
        StatsOptionEhp,
        StatsOptionErps,
        StatsOptionFitDps,
        StatsOptionFitRemoteNps,
        StatsOptionFitRemoteRps,
        StatsOptionFitVolley,
        StatsOptionRps,
    )


@dataclasses.dataclass(kw_only=True)
class FitStatsOptions:

    default: bool | type[Absent] = False
    high_slots: bool | type[Absent] = Absent
    mid_slots: bool | type[Absent] = Absent
    low_slots: bool | type[Absent] = Absent
    turret_slots: bool | type[Absent] = Absent
    launcher_slots: bool | type[Absent] = Absent
    rig_slots: bool | type[Absent] = Absent
    service_slots: bool | type[Absent] = Absent
    subsystem_slots: bool | type[Absent] = Absent
    launched_drones: bool | type[Absent] = Absent
    launched_fighters: bool | type[Absent] = Absent
    launched_light_fighters: bool | type[Absent] = Absent
    launched_heavy_fighters: bool | type[Absent] = Absent
    launched_support_fighters: bool | type[Absent] = Absent
    launched_st_light_fighters: bool | type[Absent] = Absent
    launched_st_heavy_fighters: bool | type[Absent] = Absent
    launched_st_support_fighters: bool | type[Absent] = Absent
    cpu: bool | type[Absent] = Absent
    powergrid: bool | type[Absent] = Absent
    calibration: bool | type[Absent] = Absent
    drone_bay_volume: bool | type[Absent] = Absent
    drone_bandwidth: bool | type[Absent] = Absent
    fighter_bay_volume: bool | type[Absent] = Absent
    speed: bool | type[Absent] = Absent
    agility: bool | type[Absent] = Absent
    align_time: bool | type[Absent] = Absent
    sig_radius: bool | type[Absent] = Absent
    mass: bool | type[Absent] = Absent
    warp_speed: bool | type[Absent] = Absent
    max_warp_range: bool | type[Absent] = Absent
    locks: bool | type[Absent] = Absent
    lock_range: bool | type[Absent] = Absent
    scan_res: bool | type[Absent] = Absent
    sensor: bool | type[Absent] = Absent
    dscan_range: bool | type[Absent] = Absent
    probing_size: bool | type[Absent] = Absent
    jam_chance: bool | type[Absent] = Absent
    drone_control_range: bool | type[Absent] = Absent
    dps: bool | tuple[bool, list[StatsOptionFitDps]] | type[Absent] = Absent
    volley: bool | tuple[bool, list[StatsOptionFitVolley]] | type[Absent] = Absent
    hp: bool | type[Absent] = Absent
    ehp: bool | tuple[bool, list[StatsOptionEhp]] | type[Absent] = Absent
    wc_ehp: bool | type[Absent] = Absent
    rps: bool | tuple[bool, list[StatsOptionRps]] | type[Absent] = Absent
    erps: bool | tuple[bool, list[StatsOptionErps]] | type[Absent] = Absent
    resists: bool | type[Absent] = Absent
    remote_rps: bool | tuple[bool, list[StatsOptionFitRemoteRps]] | type[Absent] = Absent
    remote_cps: bool | type[Absent] = Absent
    remote_nps: bool | tuple[bool, list[StatsOptionFitRemoteNps]] | type[Absent] = Absent
    cap_amount: bool | type[Absent] = Absent
    cap_balance: bool | tuple[bool, list[StatsOptionCapBalance]] | type[Absent] = Absent
    neut_resist: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
