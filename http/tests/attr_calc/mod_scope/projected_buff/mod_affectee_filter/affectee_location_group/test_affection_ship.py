from tests import approx


def test_affected_child_ship(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.mod_titan_effect_generator, cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 30},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_ship()
    eve_module = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(260)


def test_unaffected_other_group(client, consts):
    eve_grp1 = client.mk_eve_item_group()
    eve_grp2 = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, group_id=eve_grp1.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.mod_titan_effect_generator, cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 30},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_ship()
    eve_module = client.mk_eve_item(grp_id=eve_grp2.id, attrs={eve_affectee_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)


def test_unaffected_child_struct(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.mod_titan_effect_generator, cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 30},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_struct = client.mk_eve_struct()
    eve_module = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_ship(type_id=eve_struct.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_projs=[api_struct.id])
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)


def test_unaffected_child_char(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.mod_titan_effect_generator, cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 30},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_ship()
    eve_implant = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_char(type_id=eve_char.id)
    api_implant = api_fit.add_implant(type_id=eve_implant.id)
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_implant.update().attrs[eve_affectee_attr.id].dogma == approx(200)


def test_unaffected_root_ship(client, consts):
    # Location roots shouldn't be affected by location modifications
    eve_grp = client.mk_eve_ship()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.mod_titan_effect_generator, cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 30},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_ship(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(200)


def test_unaffected_other_fit(client, consts):
    # Check that projected modifications are not carried over to another fit
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.mod_titan_effect_generator, cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 30},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_ship()
    eve_module = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit1.set_ship(type_id=eve_ship.id)
    api_fit2.set_ship(type_id=eve_ship.id)
    api_module = api_fit2.add_mod(type_id=eve_module.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)


def test_unaffected_via_child(client, consts):
    # Check that modification is not applied directly - it can be applied only via root item
    eve_grp = client.mk_eve_ship_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.mod_titan_effect_generator, cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 30},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_ship(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 200})
    eve_module = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 200})
    eve_drone = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    api_drone = api_fit.add_drone(type_id=eve_drone.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_projs=[api_drone.id])
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    assert api_drone.update().attrs[eve_affectee_attr.id].dogma == approx(200)
