from tests import approx


def test_slots(client, consts):
    eve_affector_attr_hi = client.mk_eve_attr(id_=consts.EveAttr.hi_slot_modifier)
    eve_affector_attr_mid = client.mk_eve_attr(id_=consts.EveAttr.med_slot_modifier)
    eve_affector_attr_low = client.mk_eve_attr(id_=consts.EveAttr.low_slot_modifier)
    eve_affectee_attr_hi = client.mk_eve_attr(id_=consts.EveAttr.hi_slots)
    eve_affectee_attr_mid = client.mk_eve_attr(id_=consts.EveAttr.med_slots)
    eve_affectee_attr_low = client.mk_eve_attr(id_=consts.EveAttr.low_slots)
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.slot_modifier)
    eve_subsystem = client.mk_eve_item(
        attrs={eve_affector_attr_hi.id: 3, eve_affector_attr_mid.id: 4, eve_affector_attr_low.id: 1},
        eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_ship(
        attrs={eve_affectee_attr_hi.id: 0, eve_affectee_attr_mid.id: 2, eve_affectee_attr_low.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_subsystem = api_fit.add_subsystem(type_id=eve_subsystem.id)
    # Verification
    api_ship.update()
    # High slots
    assert api_ship.attrs[eve_affectee_attr_hi.id].dogma == approx(3)
    api_mod_hi = api_ship.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_hi.id,
        affector_item_id=api_subsystem.id).one()
    assert api_mod_hi.op == consts.ApiModOp.mod_add
    assert api_mod_hi.initial_val == approx(3)
    assert api_mod_hi.stacking_mult is None
    assert api_mod_hi.applied_val == approx(3)
    # Medium slots
    assert api_ship.attrs[eve_affectee_attr_mid.id].dogma == approx(6)
    api_mod_mid = api_ship.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_mid.id,
        affector_item_id=api_subsystem.id).one()
    assert api_mod_mid.op == consts.ApiModOp.mod_add
    assert api_mod_mid.initial_val == approx(4)
    assert api_mod_mid.stacking_mult is None
    assert api_mod_mid.applied_val == approx(4)
    # Low slots
    assert api_ship.attrs[eve_affectee_attr_low.id].dogma == approx(2)
    api_mod_low = api_ship.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_low.id,
        affector_item_id=api_subsystem.id).one()
    assert api_mod_low.op == consts.ApiModOp.mod_add
    assert api_mod_low.initial_val == approx(1)
    assert api_mod_low.stacking_mult is None
    assert api_mod_low.applied_val == approx(1)


def test_hardpoints(client, consts):
    eve_affector_attr_turret = client.mk_eve_attr(id_=consts.EveAttr.turret_hardpoint_modifier)
    eve_affector_attr_launcher = client.mk_eve_attr(id_=consts.EveAttr.launcher_hardpoint_modifier)
    eve_affectee_attr_turret = client.mk_eve_attr(id_=consts.EveAttr.turret_slots_left)
    eve_affectee_attr_launcher = client.mk_eve_attr(id_=consts.EveAttr.launcher_slots_left)
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.hardpoint_modifier_effect)
    eve_subsystem = client.mk_eve_item(
        attrs={eve_affector_attr_turret.id: 4, eve_affector_attr_launcher.id: 6},
        eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr_turret.id: 0, eve_affectee_attr_launcher.id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_subsystem = api_fit.add_subsystem(type_id=eve_subsystem.id)
    # Verification
    api_ship.update()
    # Turrets
    assert api_ship.attrs[eve_affectee_attr_turret.id].dogma == approx(4)
    api_mod_turret = api_ship.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_turret.id,
        affector_item_id=api_subsystem.id).one()
    assert api_mod_turret.op == consts.ApiModOp.mod_add
    assert api_mod_turret.initial_val == approx(4)
    assert api_mod_turret.stacking_mult is None
    assert api_mod_turret.applied_val == approx(4)
    # Launchers
    assert api_ship.attrs[eve_affectee_attr_launcher.id].dogma == approx(8)
    api_mod_launcher = api_ship.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_launcher.id,
        affector_item_id=api_subsystem.id).one()
    assert api_mod_launcher.op == consts.ApiModOp.mod_add
    assert api_mod_launcher.initial_val == approx(6)
    assert api_mod_launcher.stacking_mult is None
    assert api_mod_launcher.applied_val == approx(6)
