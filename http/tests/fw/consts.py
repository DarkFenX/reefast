from enum import Enum, IntEnum, StrEnum, unique


@unique
class EveItem(IntEnum):
    missile_launcher_operation = 3319
    high_speed_maneuvering = 3454
    micro_jump_drive_operation = 4385
    capital_ships = 20533
    nanite_repair_paste = 28668
    stasis_webification_probe = 56748
    capital_micro_jump_drive_operation = 83464


@unique
class EveItemGrp(IntEnum):
    character = 1
    effect_beacon = 920
    strategic_cruiser = 963
    ship_modifier = 1306
    mutaplasmids = 1964


@unique
class EveItemCat(IntEnum):
    celestial = 2
    ship = 6
    module = 7
    charge = 8
    skill = 16
    commodity = 17
    drone = 18
    implant = 20
    subsystem = 32
    structure = 65
    structure_module = 66
    fighter = 87


@unique
class EveEffect(IntEnum):
    lo_power = 11
    hi_power = 12
    med_power = 13
    online = 16
    launcher_fitted = 40
    turret_fitted = 42
    use_missiles = 101
    missile_em_dmg_bonus = 660
    missile_expl_dmg_bonus = 661
    missile_therm_dmg_bonus = 662
    missile_kin_dmg_bonus = 668
    drone_dmg_bonus = 1730
    self_rof = 1851
    rig_slot = 2663
    overload_self_duration_bonus = 3002
    warp_disrupt_sphere = 3380
    script_warp_scramble_range_bonus = 3648
    hardpoint_modifier_effect = 3773
    slot_modifier = 3774
    max_range_hidden_preass_warp_scramble_range = 4894
    adaptive_armor_hardener = 4928
    fueled_armor_repair = 5275
    structure_warp_scramble_block_mwd_with_npc = 6222
    doomsday_aoe_web = 6476
    fighter_ability_launch_bomb = 6485
    ship_module_arar = 6651
    mod_bonus_microwarpdrive = 6730
    mod_bonus_afterburner = 6731
    mod_bonus_warfare_link_armor = 6732
    mod_titan_effect_generator = 6753
    ship_mod_focused_warp_scrambling_script = 6848
    ship_mod_focused_warp_disruption_script = 6849
    script_standup_warp_scram = 7026
    weather_darkness = 7060
    debuff_lance = 11691


@unique
class EveEffCat(IntEnum):
    passive = 0
    active = 1
    target = 2
    area = 3
    online = 4
    overload = 5
    dungeon = 6
    system = 7


@unique
class EveModFunc(StrEnum):
    item = 'ItemModifier'
    loc = 'LocationModifier'
    loc_grp = 'LocationGroupModifier'
    loc_srq = 'LocationRequiredSkillModifier'
    own_srq = 'OwnerRequiredSkillModifier'


@unique
class EveModLoc(StrEnum):
    item = 'itemID'
    char = 'charID'
    ship = 'shipID'
    struct = 'structureID'
    tgt = 'targetID'
    other = 'otherID'


@unique
class EveModOp(IntEnum):
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
class EveBuff(IntEnum):
    warp_penalty = 4
    disallow_dock_jump = 6
    stasis_webification_burst = 27
    disallow_tether = 57
    remote_repair_impedance = 2201


@unique
class EveBuffAggrMode(StrEnum):
    min = 'Minimum'
    max = 'Maximum'


@unique
class EveBuffOp(StrEnum):
    pre_assign = 'PreAssignment'
    pre_mul = 'PreMul'
    pre_div = 'PreDiv'
    mod_add = 'ModAdd'
    mod_sub = 'ModSub'
    post_mul = 'PostMul'
    post_div = 'PostDiv'
    post_percent = 'PostPercent'
    post_assign = 'PostAssignment'


@unique
class EveAttr(IntEnum):
    mass = 4
    power_output = 11
    low_slots = 12
    med_slots = 13
    hi_slots = 14
    speed_factor = 20
    power = 30
    max_velocity = 37
    capacity = 38
    cpu_output = 48
    cpu = 50
    speed = 51
    max_range = 54
    dmg_mult = 64
    duration = 73
    armor_dmg_amount = 84
    launcher_slots_left = 101
    turret_slots_left = 102
    warp_scramble_range = 103
    warp_scramble_status = 104
    warp_scramble_strength = 105
    em_dmg = 114
    expl_dmg = 116
    kin_dmg = 117
    therm_dmg = 118
    charge_size = 128
    falloff = 158
    volume = 161
    missile_dmg_mult = 212
    armor_em_dmg_resonance = 267
    armor_expl_dmg_resonance = 268
    armor_kin_dmg_resonance = 269
    armor_therm_dmg_resonance = 270
    skill_level = 280
    drone_capacity = 283
    dmg_mult_bonus = 292
    rof_bonus = 293
    implantness = 331
    max_active_drones = 352
    sig_radius = 552
    sig_radius_bonus = 554
    speed_boost_factor = 567
    charge_group1 = 604
    charge_group2 = 605
    charge_group3 = 606
    charge_group4 = 609
    charge_group5 = 610
    max_group_active = 763
    mass_addition = 796
    disallow_assistance = 854
    max_group_online = 978
    boosterness = 1087
    upgrade_capacity = 1132
    upgrade_cost = 1153
    upgrade_slots_left = 1154
    overload_self_duration_bonus = 1206
    drone_bandwidth = 1271
    drone_bandwidth_used = 1272
    can_fit_ship_group1 = 1298
    can_fit_ship_group2 = 1299
    can_fit_ship_group3 = 1300
    can_fit_ship_group4 = 1301
    can_fit_ship_type1 = 1302
    can_fit_ship_type2 = 1303
    can_fit_ship_type3 = 1304
    can_fit_ship_type4 = 1305
    max_range_hidden = 1317
    warp_scramble_range_bonus = 1327
    activation_blocked = 1349
    activation_blocked_strength = 1350
    subsystem_slot = 1366
    max_subsystems = 1367
    turret_hardpoint_modifier = 1368
    launcher_hardpoint_modifier = 1369
    hi_slot_modifier = 1374
    med_slot_modifier = 1375
    low_slot_modifier = 1376
    fits_to_ship_type = 1380
    armor_max_dmg_resonance = 1527
    max_group_fitted = 1544
    rig_size = 1547
    resist_shift_amount = 1849
    charged_armor_dmg_mult = 1886
    can_fit_ship_type5 = 1944
    gate_scramble_status = 1973
    gate_scramble_strength = 1974
    can_fit_ship_group9 = 2065
    can_fit_ship_type6 = 2103
    remote_resistance_id = 2138
    ftr_tubes = 2216
    ftr_light_slots = 2217
    ftr_support_slots = 2218
    ftr_heavy_slots = 2219
    ftr_sq_is_light = 2212
    ftr_sq_is_support = 2213
    ftr_sq_is_heavy = 2214
    doomsday_aoe_range = 2279
    ftr_abil_launch_bomb_type = 2324
    can_fit_ship_group10 = 2396
    can_fit_ship_type7 = 2463
    warfare_buff_1_id = 2468
    warfare_buff_1_value = 2469
    warfare_buff_2_id = 2470
    warfare_buff_2_value = 2471
    warfare_buff_3_id = 2472
    warfare_buff_3_value = 2473
    can_fit_ship_group11 = 2476
    can_fit_ship_group12 = 2477
    can_fit_ship_group13 = 2478
    can_fit_ship_group14 = 2479
    can_fit_ship_group15 = 2480
    can_fit_ship_group16 = 2481
    can_fit_ship_group17 = 2482
    can_fit_ship_group18 = 2483
    can_fit_ship_group19 = 2484
    can_fit_ship_group20 = 2485
    can_fit_ship_type8 = 2486
    can_fit_ship_type9 = 2487
    can_fit_ship_type10 = 2488
    warfare_buff_4_id = 2536
    warfare_buff_4_value = 2537
    ftr_standup_light_slots = 2737
    ftr_standup_support_slots = 2738
    ftr_standup_heavy_slots = 2739
    ftr_sq_is_standup_light = 2740
    ftr_sq_is_standup_support = 2741
    ftr_sq_is_standup_heavy = 2742
    can_fit_ship_type11 = 2758


@unique
class EveAttrUnit(IntEnum):
    group_id = 115
    item_id = 116


@unique
class ApiItemKind(StrEnum):
    autocharge = 'autocharge'
    booster = 'booster'
    character = 'character'
    charge = 'charge'
    drone = 'drone'
    fighter = 'fighter'
    fw_effect = 'fw_effect'
    implant = 'implant'
    module = 'module'
    proj_effect = 'proj_effect'
    rig = 'rig'
    ship = 'ship'
    skill = 'skill'
    stance = 'stance'
    subsystem = 'subsystem'
    sw_effect = 'sw_effect'


@unique
class ApiModuleState(StrEnum):
    ghost = 'ghost'
    offline = 'offline'
    online = 'online'
    active = 'active'
    overload = 'overload'


@unique
class ApiMinionState(StrEnum):
    in_bay = 'in_bay'
    in_space = 'in_space'
    engaging = 'engaging'


@unique
class ApiRack(StrEnum):
    high = 'high'
    mid = 'mid'
    low = 'low'


@unique
class ApiAttrMutation(StrEnum):
    roll = 'roll'
    absolute = 'absolute'


@unique
class ApiEffMode(StrEnum):
    full_compliance = 'full'
    state_compliance = 'state'
    force_run = 'run'
    force_stop = 'stop'


@unique
class ApiModOp(StrEnum):
    pre_assign = 'pre_assign'
    pre_mul = 'pre_mul'
    pre_div = 'pre_div'
    mod_add = 'add'
    mod_sub = 'sub'
    post_mul = 'post_mul'
    post_div = 'post_div'
    post_percent = 'post_perc'
    post_assign = 'post_assign'
    min_limit = 'min_limit'
    max_limit = 'max_limit'
    extra_mul = 'extra_mul'


@unique
class ApiAggrMode(StrEnum):
    stack = 'stack'


@unique
class ApiModAddMode(StrEnum):
    append = 'append'
    equip = 'equip'
    insert = 'insert'
    replace = 'replace'


@unique
class ApiModRmMode(StrEnum):
    remove = 'remove'
    free = 'free'


@unique
class ApiSideEffectOp(StrEnum):
    add = 'add'
    perc = 'perc'


@unique
class ApiValType(StrEnum):
    cpu = 'cpu'
    powergrid = 'powergrid'
    calibration = 'calibration'
    dronebay_volume = 'dronebay_volume'
    drone_bandwidth = 'drone_bandwidth'
    rig_slots = 'rig_slots'
    subsystem_slots = 'subsystem_slots'
    launched_drones = 'launched_drones'
    launched_fighters = 'launched_fighters'
    launched_support_fighters = 'launched_support_fighters'
    launched_light_fighters = 'launched_light_fighters'
    launched_heavy_fighters = 'launched_heavy_fighters'
    launched_standup_support_fighters = 'launched_standup_support_fighters'
    launched_standup_light_fighters = 'launched_standup_light_fighters'
    launched_standup_heavy_fighters = 'launched_standup_heavy_fighters'
    turret_slots = 'turret_slots'
    launcher_slots = 'launcher_slots'
    high_slots = 'high_slots'
    mid_slots = 'mid_slots'
    low_slots = 'low_slots'
    implant_slot_index = 'implant_slot_index'
    booster_slot_index = 'booster_slot_index'
    subsystem_slot_index = 'subsystem_slot_index'
    ship_limit = 'ship_limit'
    max_group_fitted = 'max_group_fitted'
    max_group_online = 'max_group_online'
    max_group_active = 'max_group_active'
    rig_size = 'rig_size'
    skill_reqs = 'skill_reqs'
    charge_group = 'charge_group'
    charge_size = 'charge_size'
    charge_volume = 'charge_volume'
    capital_module = 'capital_module'
    not_loaded_item = 'not_loaded_item'
    module_state = 'module_state'
    item_kind = 'item_kind'


@unique
class ApiValItemType(StrEnum):
    booster = 'booster'
    character = 'character'
    charge = 'charge'
    drone = 'drone'
    fighter = 'fighter'
    implant = 'implant'
    module_high = 'module_high'
    module_mid = 'module_mid'
    module_low = 'module_low'
    rig = 'rig'
    ship = 'ship'
    skill = 'skill'
    stance = 'stance'
    subsystem = 'subsystem'


@unique
class ApiSolInfoMode(StrEnum):
    id = 'id'
    full = 'full'


@unique
class ApiFitInfoMode(StrEnum):
    id = 'id'
    full = 'full'


@unique
class ApiFleetInfoMode(StrEnum):
    id = 'id'
    full = 'full'


@unique
class ApiItemInfoMode(StrEnum):
    id = 'id'
    partial = 'partial'
    full = 'full'


@unique
class ApiValInfoMode(StrEnum):
    simple = 'simple'
    detailed = 'detailed'


@unique
class PenaltyStr(float, Enum):
    p1 = 1
    p2 = 0.8691199808003977
    p3 = 0.5705831435105605
    p4 = 0.2829551540232615
    p5 = 0.10599264974270461
    p6 = 0.02999116653328059
    p7 = 0.006410183117533543
    p8 = 0.0010349204826687127
    p9 = 0.0001262126825458958
    p10 = 0.000011626753929631052
    p11 = 0.0000008090464068743218
