from pytest import approx


def test_ab(client, consts):
    eve_speed_attr = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_thrust_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_boost_factor)
    eve_speed_boost_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_mass_attr = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mass_add_attr = client.mk_eve_attr(id_=consts.EveAttr.mass_addition)
    eve_sig_src_attr = client.mk_eve_attr(id_=consts.EveAttr.sig_radius_bonus)
    eve_sig_tgt_attr = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_prop_effect = client.mk_eve_effect(id_=consts.EveEffect.mod_bonus_afterburner, cat_id=consts.EveEffCat.active)
    eve_ship_item = client.mk_eve_item(
        attrs={eve_speed_attr.id: 455, eve_mass_attr.id: 1050000, eve_sig_tgt_attr.id: 32})
    eve_prop_item = client.mk_eve_item(
        attrs={
            eve_speed_boost_attr.id: 135, eve_thrust_attr.id: 1500000,
            eve_mass_add_attr.id: 500000, eve_sig_src_attr.id: 200},
        eff_ids=[eve_prop_effect.id],
        defeff_id=eve_prop_effect.id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship_item = api_fit.set_ship(type_id=eve_ship_item.id)
    api_prop_item = api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    # Verification
    api_ship_item.update()
    assert api_ship_item.attrs[eve_mass_attr.id].dogma == approx(1550000)
    assert api_ship_item.attrs[eve_sig_tgt_attr.id].dogma == approx(32)  # Not affected by sig blow
    assert api_ship_item.attrs[eve_speed_attr.id].dogma == approx(1049.43548)
    assert len(api_ship_item.mods) == 2
    # Mass modification
    api_mod_mass = api_ship_item.mods.find_by_src_attr(
        tgt_attr_id=eve_mass_attr.id, src_attr_id=eve_mass_add_attr.id).one()
    assert api_mod_mass.val == approx(500000)
    assert api_mod_mass.op == consts.ApiModOp.mod_add
    assert api_mod_mass.src.one().item_id == api_prop_item.id
    assert api_mod_mass.src.one().attr_id == eve_mass_add_attr.id
    # Speed modification
    api_mod_prop = api_ship_item.mods.find_by_src_attr(
        tgt_attr_id=eve_speed_attr.id, src_attr_id=eve_speed_boost_attr.id).one()
    assert api_mod_prop.val == approx(2.30645)
    assert api_mod_prop.op == consts.ApiModOp.post_mul
    assert len(api_mod_prop.src) == 3
    assert api_mod_prop.src.find_by_attr(attr_id=eve_speed_boost_attr.id).one().item_id == api_prop_item.id
    assert api_mod_prop.src.find_by_attr(attr_id=eve_thrust_attr.id).one().item_id == api_prop_item.id
    assert api_mod_prop.src.find_by_attr(attr_id=eve_mass_attr.id).one().item_id == api_ship_item.id


def test_mwd(client, consts):
    eve_speed_attr = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_thrust_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_boost_factor)
    eve_speed_boost_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_mass_attr = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mass_add_attr = client.mk_eve_attr(id_=consts.EveAttr.mass_addition)
    eve_sig_src_attr = client.mk_eve_attr(id_=consts.EveAttr.sig_radius_bonus)
    eve_sig_tgt_attr = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_prop_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_microwarpdrive,
        cat_id=consts.EveEffCat.active)
    eve_ship_item = client.mk_eve_item(
        attrs={eve_speed_attr.id: 455, eve_mass_attr.id: 1050000, eve_sig_tgt_attr.id: 32})
    eve_prop_item = client.mk_eve_item(
        attrs={
            eve_speed_boost_attr.id: 505, eve_thrust_attr.id: 1500000,
            eve_mass_add_attr.id: 500000, eve_sig_src_attr.id: 450},
        eff_ids=[eve_prop_effect.id],
        defeff_id=eve_prop_effect.id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship_item = api_fit.set_ship(type_id=eve_ship_item.id)
    api_prop_item = api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    # Verification
    api_ship_item.update()
    assert api_ship_item.attrs[eve_mass_attr.id].dogma == approx(1550000)
    assert api_ship_item.attrs[eve_sig_tgt_attr.id].dogma == approx(176)
    assert api_ship_item.attrs[eve_speed_attr.id].dogma == approx(2678.62903)
    assert len(api_ship_item.mods) == 3
    # Mass modification
    api_mod_mass = api_ship_item.mods.find_by_src_attr(
        tgt_attr_id=eve_mass_attr.id, src_attr_id=eve_mass_add_attr.id).one()
    assert api_mod_mass.val == approx(500000)
    assert api_mod_mass.op == consts.ApiModOp.mod_add
    assert api_mod_mass.src.one().item_id == api_prop_item.id
    assert api_mod_mass.src.one().attr_id == eve_mass_add_attr.id
    # Sig modification
    api_mod_sig = api_ship_item.mods.find_by_src_attr(
        tgt_attr_id=eve_sig_tgt_attr.id, src_attr_id=eve_sig_src_attr.id).one()
    assert api_mod_sig.val == approx(450)
    assert api_mod_sig.op == consts.ApiModOp.post_percent
    assert api_mod_sig.src.one().item_id == api_prop_item.id
    assert api_mod_sig.src.one().attr_id == eve_sig_src_attr.id
    # Speed modification
    api_mod_prop = api_ship_item.mods.find_by_src_attr(
        tgt_attr_id=eve_speed_attr.id, src_attr_id=eve_speed_boost_attr.id).one()
    assert api_mod_prop.val == approx(5.8871)
    assert api_mod_prop.op == consts.ApiModOp.post_mul
    assert len(api_mod_prop.src) == 3
    assert api_mod_prop.src.find_by_attr(attr_id=eve_speed_boost_attr.id).one().item_id == api_prop_item.id
    assert api_mod_prop.src.find_by_attr(attr_id=eve_thrust_attr.id).one().item_id == api_prop_item.id
    assert api_mod_prop.src.find_by_attr(attr_id=eve_mass_attr.id).one().item_id == api_ship_item.id


def test_state(client, consts):
    eve_speed_attr = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_thrust_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_boost_factor)
    eve_speed_boost_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_mass_attr = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mass_add_attr = client.mk_eve_attr(id_=consts.EveAttr.mass_addition)
    eve_sig_src_attr = client.mk_eve_attr(id_=consts.EveAttr.sig_radius_bonus)
    eve_sig_tgt_attr = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_prop_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_microwarpdrive,
        cat_id=consts.EveEffCat.active)
    eve_ship_item = client.mk_eve_item(
        attrs={eve_speed_attr.id: 455, eve_mass_attr.id: 1050000, eve_sig_tgt_attr.id: 32})
    eve_prop_item = client.mk_eve_item(
        attrs={
            eve_speed_boost_attr.id: 505, eve_thrust_attr.id: 1500000,
            eve_mass_add_attr.id: 500000, eve_sig_src_attr.id: 450},
        eff_ids=[eve_prop_effect.id],
        defeff_id=eve_prop_effect.id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship_item = api_fit.set_ship(type_id=eve_ship_item.id)
    api_prop = api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    # Verification
    api_ship_item.update()
    assert api_ship_item.attrs[eve_mass_attr.id].dogma == approx(1550000)
    assert api_ship_item.attrs[eve_sig_tgt_attr.id].dogma == approx(176)
    assert api_ship_item.attrs[eve_speed_attr.id].dogma == approx(2678.62903)
    assert len(api_ship_item.mods) == 3
    # Action
    api_prop.change_mod(state=consts.ApiState.online)
    # Verification
    api_ship_item.update()
    assert api_ship_item.attrs[eve_mass_attr.id].dogma == approx(1050000)
    assert api_ship_item.attrs[eve_sig_tgt_attr.id].dogma == approx(32)
    assert api_ship_item.attrs[eve_speed_attr.id].dogma == approx(455)
    assert len(api_ship_item.mods) == 0


def test_speed_mod_stacking(client, consts):
    # Actual EVE scenario, AB speed boost + black hole speed boost
    eve_speed_attr = client.mk_eve_attr(id_=consts.EveAttr.max_velocity, stackable=False)
    eve_thrust_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_boost_factor)
    eve_speed_boost_attr_prop = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_mass_attr = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mass_add_attr = client.mk_eve_attr(id_=consts.EveAttr.mass_addition)
    eve_prop_effect = client.mk_eve_effect(id_=consts.EveEffect.mod_bonus_afterburner, cat_id=consts.EveEffCat.active)
    eve_ship_item = client.mk_eve_item(attrs={eve_speed_attr.id: 455, eve_mass_attr.id: 1050000})
    eve_prop_item = client.mk_eve_item(
        attrs={eve_speed_boost_attr_prop.id: 135, eve_thrust_attr.id: 1500000, eve_mass_add_attr.id: 500000},
        eff_ids=[eve_prop_effect.id],
        defeff_id=eve_prop_effect.id)
    eve_speed_boost_attr_sw = client.mk_eve_attr()
    eve_sw_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_speed_boost_attr_sw.id,
        tgt_attr_id=eve_speed_attr.id)
    eve_sw_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_sw_mod])
    eve_sw_item = client.mk_eve_item(attrs={eve_speed_boost_attr_sw.id: 1.86}, eff_ids=[eve_sw_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship_item = api_fit.set_ship(type_id=eve_ship_item.id)
    api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    api_sol.add_sw_effect(type_id=eve_sw_item.id)
    # Verification - if prop speed boost wasn't penalized against BH speed boost, speed would be
    # 1951.95
    api_ship_item.update()
    assert api_ship_item.attrs[eve_speed_attr.id].dogma == approx(1833.82888)
    api_ship_mod_prop = api_ship_item.mods.find_by_src_attr(
        tgt_attr_id=eve_speed_attr.id, src_attr_id=eve_speed_boost_attr_prop.id).one()
    assert api_ship_mod_prop.val == approx(2.30645)
    assert api_ship_mod_prop.op == consts.ApiModOp.post_mul
    assert api_ship_mod_prop.penalized is True
    api_ship_mod_sw = api_ship_item.mods.find_by_src_attr(
        tgt_attr_id=eve_speed_attr.id, src_attr_id=eve_speed_boost_attr_sw.id).one()
    assert api_ship_mod_sw.val == approx(1.86)
    assert api_ship_mod_sw.op == consts.ApiModOp.post_mul
    assert api_ship_mod_sw.penalized is True


def test_sig_mod_stacking(client, consts):
    # Actual EVE scenario, MWD sig bloom + shield rigs
    eve_sig_src_attr_prop = client.mk_eve_attr(id_=consts.EveAttr.sig_radius_bonus)
    eve_sig_tgt_attr = client.mk_eve_attr(id_=consts.EveAttr.sig_radius, stackable=False)
    eve_prop_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_microwarpdrive,
        cat_id=consts.EveEffCat.active)
    eve_ship_item = client.mk_eve_item(attrs={eve_sig_tgt_attr.id: 32})
    eve_prop_item = client.mk_eve_item(
        attrs={eve_sig_src_attr_prop.id: 450},
        eff_ids=[eve_prop_effect.id],
        defeff_id=eve_prop_effect.id)
    eve_sig_src_attr_rig = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_sig_src_attr_rig.id,
        tgt_attr_id=eve_sig_tgt_attr.id)
    eve_rig_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_item = client.mk_eve_item(attrs={eve_sig_src_attr_rig.id: 10}, eff_ids=[eve_rig_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship_item = api_fit.set_ship(type_id=eve_ship_item.id)
    api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    api_fit.add_rig(type_id=eve_rig_item.id)
    # Verification - if MWD sig bloom wasn't stacking penalized against rig sig penalty, it'd be
    # 193.6
    api_ship_item.update()
    assert api_ship_item.attrs[eve_sig_tgt_attr.id].dogma == approx(191.29651)
    api_ship_mod_prop = api_ship_item.mods.find_by_src_attr(
        tgt_attr_id=eve_sig_tgt_attr.id, src_attr_id=eve_sig_src_attr_prop.id).one()
    assert api_ship_mod_prop.val == approx(450)
    assert api_ship_mod_prop.op == consts.ApiModOp.post_percent
    assert api_ship_mod_prop.penalized is True
    api_ship_mod_rig = api_ship_item.mods.find_by_src_attr(
        tgt_attr_id=eve_sig_tgt_attr.id, src_attr_id=eve_sig_src_attr_rig.id).one()
    assert api_ship_mod_rig.val == approx(10)
    assert api_ship_mod_rig.op == consts.ApiModOp.post_percent
    assert api_ship_mod_rig.penalized is True


def test_speed_mod_mass_zero(client, consts):
    # Part of speed boost calculation is division by mass, just check what happens if it's 0
    eve_speed_attr = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_thrust_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_boost_factor)
    eve_speed_boost_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_mass_attr = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_prop_effect = client.mk_eve_effect(id_=consts.EveEffect.mod_bonus_afterburner, cat_id=consts.EveEffCat.active)
    eve_ship_item = client.mk_eve_item(attrs={eve_speed_attr.id: 455, eve_mass_attr.id: 0})
    eve_prop_item = client.mk_eve_item(
        attrs={eve_speed_boost_attr.id: 135, eve_thrust_attr.id: 1500000},
        eff_ids=[eve_prop_effect.id],
        defeff_id=eve_prop_effect.id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship_item = api_fit.set_ship(type_id=eve_ship_item.id)
    api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    # Verification
    api_ship_item.update()
    assert api_ship_item.attrs[eve_mass_attr.id].dogma == approx(0)
    assert api_ship_item.attrs[eve_speed_attr.id].dogma == approx(455)
    assert len(api_ship_item.mods) == 0


def test_speed_mod_mass_changed(client, consts):
    eve_speed_attr = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_thrust_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_boost_factor)
    eve_speed_boost_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_mass_attr = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mass_add_attr = client.mk_eve_attr(id_=consts.EveAttr.mass_addition)
    eve_prop_effect = client.mk_eve_effect(id_=consts.EveEffect.mod_bonus_afterburner, cat_id=consts.EveEffCat.active)
    eve_ship1_item = client.mk_eve_item(attrs={eve_speed_attr.id: 455, eve_mass_attr.id: 1050000})
    eve_ship2_item = client.mk_eve_item(attrs={eve_speed_attr.id: 420, eve_mass_attr.id: 1060000})
    eve_prop_item = client.mk_eve_item(
        attrs={eve_speed_boost_attr.id: 135, eve_thrust_attr.id: 1500000, eve_mass_add_attr.id: 500000},
        eff_ids=[eve_prop_effect.id],
        defeff_id=eve_prop_effect.id)
    eve_mass_boost_attr = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_mass_boost_attr.id,
        tgt_attr_id=eve_mass_attr.id)
    eve_rig_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_item = client.mk_eve_item(attrs={eve_mass_boost_attr.id: 100}, eff_ids=[eve_rig_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship1_item = api_fit.set_ship(type_id=eve_ship1_item.id)
    api_prop_item = api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_mass_attr.id].dogma == approx(1550000)
    assert api_ship1_item.attrs[eve_speed_attr.id].dogma == approx(1049.43548)
    assert len(api_ship1_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1
    # Action
    api_rig_item = api_fit.add_rig(type_id=eve_rig_item.id)
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_mass_attr.id].dogma == approx(3100000)
    assert api_ship1_item.attrs[eve_speed_attr.id].dogma == approx(752.21774)
    assert len(api_ship1_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1
    # Action
    api_rig_item.remove()
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_mass_attr.id].dogma == approx(1550000)
    assert api_ship1_item.attrs[eve_speed_attr.id].dogma == approx(1049.43548)
    assert len(api_ship1_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1
    # Action - make sure it works even after we switch ship
    api_ship2_item = api_fit.set_ship(type_id=eve_ship2_item.id)
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_mass_attr.id].dogma == approx(1560000)
    assert api_ship2_item.attrs[eve_speed_attr.id].dogma == approx(965.19231)
    assert len(api_ship2_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1
    # Action
    api_rig_item = api_fit.add_rig(type_id=eve_rig_item.id)
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_mass_attr.id].dogma == approx(3120000)
    assert api_ship2_item.attrs[eve_speed_attr.id].dogma == approx(692.59615)
    assert len(api_ship2_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1
    # Action
    api_rig_item.remove()
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_mass_attr.id].dogma == approx(1560000)
    assert api_ship2_item.attrs[eve_speed_attr.id].dogma == approx(965.19231)
    assert len(api_ship2_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1


def test_speed_mod_boost_changed(client, consts):
    eve_speed_attr = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_thrust_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_boost_factor)
    eve_speed_boost_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_mass_attr = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mass_add_attr = client.mk_eve_attr(id_=consts.EveAttr.mass_addition)
    eve_prop_effect = client.mk_eve_effect(id_=consts.EveEffect.mod_bonus_afterburner, cat_id=consts.EveEffCat.active)
    eve_ship1_item = client.mk_eve_item(attrs={eve_speed_attr.id: 455, eve_mass_attr.id: 1050000})
    eve_ship2_item = client.mk_eve_item(attrs={eve_speed_attr.id: 420, eve_mass_attr.id: 1060000})
    eve_prop_item = client.mk_eve_item(
        attrs={eve_speed_boost_attr.id: 135, eve_thrust_attr.id: 1500000, eve_mass_add_attr.id: 500000},
        eff_ids=[eve_prop_effect.id],
        defeff_id=eve_prop_effect.id)
    eve_boost_booster_attr = client.mk_eve_attr()
    eve_implant_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_boost_booster_attr.id,
        tgt_attr_id=eve_speed_boost_attr.id)
    eve_implant_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_implant_mod])
    eve_implant_item = client.mk_eve_item(attrs={eve_boost_booster_attr.id: 10}, eff_ids=[eve_implant_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship1_item = api_fit.set_ship(type_id=eve_ship1_item.id)
    api_prop_item = api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_speed_attr.id].dogma == approx(1049.43548)
    assert len(api_ship1_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_speed_boost_attr.id].dogma == approx(135)
    # Action
    api_implant_item = api_fit.add_implant(type_id=eve_implant_item.id)
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_speed_attr.id].dogma == approx(1108.87903)
    assert len(api_ship1_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_speed_boost_attr.id].dogma == approx(148.5)
    # Action
    api_implant_item.remove()
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_speed_attr.id].dogma == approx(1049.43548)
    assert len(api_ship1_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_speed_boost_attr.id].dogma == approx(135)
    # Action - make sure it works even after we switch ship
    api_ship2_item = api_fit.set_ship(type_id=eve_ship2_item.id)
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_speed_attr.id].dogma == approx(965.19231)
    assert len(api_ship2_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_speed_boost_attr.id].dogma == approx(135)
    # Action
    api_implant_item = api_fit.add_implant(type_id=eve_implant_item.id)
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_speed_attr.id].dogma == approx(1019.71154)
    assert len(api_ship2_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_speed_boost_attr.id].dogma == approx(148.5)
    # Action
    api_implant_item.remove()
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_speed_attr.id].dogma == approx(965.19231)
    assert len(api_ship2_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_speed_boost_attr.id].dogma == approx(135)


def test_speed_mod_thrust_changed(client, consts):
    eve_speed_attr = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_thrust_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_boost_factor)
    eve_speed_boost_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_mass_attr = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mass_add_attr = client.mk_eve_attr(id_=consts.EveAttr.mass_addition)
    eve_prop_effect = client.mk_eve_effect(id_=consts.EveEffect.mod_bonus_afterburner, cat_id=consts.EveEffCat.active)
    eve_ship1_item = client.mk_eve_item(attrs={eve_speed_attr.id: 455, eve_mass_attr.id: 1050000})
    eve_ship2_item = client.mk_eve_item(attrs={eve_speed_attr.id: 420, eve_mass_attr.id: 1060000})
    eve_prop_item = client.mk_eve_item(
        attrs={eve_speed_boost_attr.id: 135, eve_thrust_attr.id: 1500000, eve_mass_add_attr.id: 500000},
        eff_ids=[eve_prop_effect.id],
        defeff_id=eve_prop_effect.id)
    eve_thrust_booster_attr = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_thrust_booster_attr.id,
        tgt_attr_id=eve_thrust_attr.id)
    eve_rig_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_item = client.mk_eve_item(attrs={eve_thrust_booster_attr.id: 30}, eff_ids=[eve_rig_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship1_item = api_fit.set_ship(type_id=eve_ship1_item.id)
    api_prop_item = api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_speed_attr.id].dogma == approx(1049.43548)
    assert len(api_ship1_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_thrust_attr.id].dogma == approx(1500000)
    # Action
    api_rig_item = api_fit.add_implant(type_id=eve_rig_item.id)
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_speed_attr.id].dogma == approx(1227.76613)
    assert len(api_ship1_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_thrust_attr.id].dogma == approx(1950000)
    # Action
    api_rig_item.remove()
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_speed_attr.id].dogma == approx(1049.43548)
    assert len(api_ship1_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_thrust_attr.id].dogma == approx(1500000)
    # Action - make sure it works even after we switch ship
    api_ship2_item = api_fit.set_ship(type_id=eve_ship2_item.id)
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_speed_attr.id].dogma == approx(965.19231)
    assert len(api_ship2_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_thrust_attr.id].dogma == approx(1500000)
    # Action
    api_rig_item = api_fit.add_implant(type_id=eve_rig_item.id)
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_speed_attr.id].dogma == approx(1128.75)
    assert len(api_ship2_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_thrust_attr.id].dogma == approx(1950000)
    # Action
    api_rig_item.remove()
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_speed_attr.id].dogma == approx(965.19231)
    assert len(api_ship2_item.mods[eve_speed_attr.id].find_by_src_item(src_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_thrust_attr.id].dogma == approx(1500000)
