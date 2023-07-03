# pylint: disable=C0103
from enum import StrEnum, IntEnum, unique


@unique
class State(StrEnum):
    ghost = 'ghost'
    offline = 'offline'
    online = 'online'
    active = 'active'
    overload = 'overload'


@unique
class Item(IntEnum):
    missile_launcher_operation = 3319


@unique
class ItemGrp(IntEnum):
    character = 1
    effect_beacon = 920


@unique
class ItemCat(IntEnum):
    celestial = 2
    charge = 8
    drone = 18
    fighter = 87
    implant = 20
    module = 7
    ship = 6
    skill = 16
    subsystem = 32


@unique
class Effect(IntEnum):
    drone_dmg_bonus = 1730
    hardpoint_modifier_effect = 3773
    missile_em_dmg_bonus = 660
    missile_expl_dmg_bonus = 661
    missile_kin_dmg_bonus = 668
    missile_therm_dmg_bonus = 662
    online = 16
    self_rof = 1851
    slot_modifier = 3774


@unique
class EffCat(IntEnum):
    passive = 0
    active = 1
    target = 2
    area = 3
    online = 4
    overload = 5
    dungeon = 6
    system = 7


@unique
class ModFunc(StrEnum):
    item = 'ItemModifier'
    loc = 'LocationModifier'
    loc_grp = 'LocationGroupModifier'
    loc_srq = 'LocationRequiredSkillModifier'
    own_srq = 'OwnerRequiredSkillModifier'


@unique
class ModDom(StrEnum):
    item = 'itemID'
    char = 'charID'
    ship = 'shipID'
    struct = 'structureID'
    tgt = 'targetID'
    other = 'otherID'


@unique
class ModOp(IntEnum):
    pre_assign = -1
    pre_mul = 0
    pre_div = 1
    mod_add = 2
    mod_sub = 3
    post_mul = 4
    post_div = 5
    post_percent = 6
    post_assign = 7


@unique
class Attr(IntEnum):
    # Resources
    cpu = 50
    cpu_output = 48
    power = 30
    power_output = 11
    # Damage
    em_dmg = 114
    therm_dmg = 118
    kin_dmg = 117
    expl_dmg = 116
    # Slots
    hi_slots = 14
    hi_slot_modifier = 1374
    med_slots = 13
    med_slot_modifier = 1375
    low_slots = 12
    low_slot_modifier = 1376
    turret_slots_left = 102
    turret_hardpoint_modifier = 1368
    launcher_slots_left = 101
    launcher_hardpoint_modifier = 1369
    # Misc
    skill_level = 280
    speed = 51
    rof_bonus = 293
    dmg_mult = 64
    dmg_mult_bonus = 292
    missile_dmg_mult = 212


@unique
class Rack(StrEnum):
    high = 'high'
    mid = 'mid'
    low = 'low'


@unique
class EffMode(StrEnum):
    full_compliance = 'full'
    state_compliance = 'state'
    force_run = 'run'
    force_stop = 'stop'
