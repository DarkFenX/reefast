from pytest import approx


def test_add_pe_item_proj_remove_state_proj_fit(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_module = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 7.5})
    eve_struct = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_ship(type_id=eve_struct.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(add_tgts=[api_struct.id])
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    api_proj_effect.change_proj_effect(state=False)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(rm_tgts=[api_struct.id])
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_fit.remove()
    api_struct.update(status_code=404)
    api_proj_effect.remove()


def test_add_item_pe_proj_state_remove_pe_item(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_module = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 7.5})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id, state=False)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(state=True)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    api_proj_effect.remove()
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_module.remove()
    api_module.update(status_code=404)
    api_fit.remove()


def test_add_item_pe_proj_remove_root_item(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_module = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 7.5})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    api_ship.remove()
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_module.remove()
    api_module.update(status_code=404)
    api_proj_effect.remove()
    api_fit.remove()
