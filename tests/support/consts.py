# pylint: disable=C0103
from enum import IntEnum, unique


@unique
class ItemCat(IntEnum):
    charge = 8
    drone = 18
    fighter = 87
    implant = 20
    module = 7
    ship = 6
    skill = 16
    subsystem = 32


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
