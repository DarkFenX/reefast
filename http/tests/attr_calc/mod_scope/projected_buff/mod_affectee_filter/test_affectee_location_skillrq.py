from pytest import approx


def test_affected_child_ship(client, consts):
    # Make sure ship items (such as modules) are affected by location-skillreq-filtered buff
    # modification
    eve_skill = client.mk_eve_item()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, skill_id=eve_skill.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item()
    eve_module = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5}, srqs={eve_skill.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_module.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_proj_effect.remove()
    assert api_module.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_affected_child_struct(client, consts):
    # Make sure structure items (such as modules) are affected by location-skillreq-filtered buff
    # modification
    eve_skill = client.mk_eve_item()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, skill_id=eve_skill.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_struct = client.mk_eve_item()
    eve_module = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5}, srqs={eve_skill.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_struct(type_id=eve_struct.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_struct.id])
    assert api_module.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_proj_effect.remove()
    assert api_module.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_unaffected_other_skillrq(client, consts):
    # Check that entities which don't have needed skill requirement are not affected
    eve_skill1 = client.mk_eve_item()
    eve_skill2 = client.mk_eve_item()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, skill_id=eve_skill1.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item()
    eve_module = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5}, srqs={eve_skill2.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_module.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_unaffected_child_of_non_buff_modifiable_parent(client, consts):
    # Character isn't buff-modifiable, so items which are located on it are not affected
    eve_skill = client.mk_eve_item()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, skill_id=eve_skill.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_item()
    eve_implant = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5}, srqs={eve_skill.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_char(type_id=eve_char.id)
    api_implant = api_fit.add_implant(type_id=eve_implant.id)
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_implant.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_unaffected_parent_ship(client, consts):
    # Location owners shouldn't be affected by location modifications
    eve_skill = client.mk_eve_item()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, skill_id=eve_skill.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5}, srqs={eve_skill.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_unaffected_parent_struct(client, consts):
    # Location owners shouldn't be affected by location modifications
    eve_skill = client.mk_eve_item()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, skill_id=eve_skill.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_struct = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5}, srqs={eve_skill.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_struct(type_id=eve_struct.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_struct.id])
    assert api_struct.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_unaffected_parent_char(client, consts):
    # Location owners shouldn't be affected by location modifications
    eve_skill = client.mk_eve_item()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, skill_id=eve_skill.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_char = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5}, srqs={eve_skill.id: 1})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_char = api_fit.set_char(type_id=eve_char.id)
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_char.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_unaffected_other_fit(client, consts):
    # Check that projected modifications are not carried over to another fit
    eve_skill = client.mk_eve_item()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, skill_id=eve_skill.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item()
    eve_module = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5}, srqs={eve_skill.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit1.set_ship(type_id=eve_ship.id)
    api_fit2.set_ship(type_id=eve_ship.id)
    api_module = api_fit2.add_mod(type_id=eve_module.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_module.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_unaffected_via_child(client, consts):
    # Check that modification is not applied directly - it can be applied only via parent item
    eve_skill = client.mk_eve_item()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, skill_id=eve_skill.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5}, srqs={eve_skill.id: 1})
    eve_module = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5}, srqs={eve_skill.id: 1})
    eve_drone = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5}, srqs={eve_skill.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    api_drone = api_fit.add_drone(type_id=eve_drone.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_drone.id])
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    assert api_module.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    assert api_drone.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_replace_parent_ship(client, consts):
    # Modifiers which target items on ship location shouldn't apply when ship isn't set
    eve_skill = client.mk_eve_item()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, skill_id=eve_skill.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item()
    eve_module = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5}, srqs={eve_skill.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship1 = api_fit.set_ship(type_id=eve_ship.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship1.id])
    assert api_module.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_ship1.remove()
    assert api_module.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    api_ship2 = api_fit.set_ship(type_id=eve_ship.id)
    assert api_module.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship2.id])
    assert api_module.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)


def test_replace_parent_struct(client, consts):
    # Modifiers which target items on structure location shouldn't apply when structure isn't set
    eve_skill = client.mk_eve_item()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id, skill_id=eve_skill.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_struct = client.mk_eve_item()
    eve_module = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5}, srqs={eve_skill.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct1 = api_fit.set_struct(type_id=eve_struct.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_struct1.id])
    assert api_module.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_struct1.remove()
    assert api_module.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    api_struct2 = api_fit.set_struct(type_id=eve_struct.id)
    assert api_module.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(add_tgts=[api_struct2.id])
    assert api_module.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
