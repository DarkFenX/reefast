from tests import approx


def test_replace_root_ship_to_ship(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, group_id=eve_grp_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    eve_module_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_ship.remove()
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_fit.set_ship(type_id=eve_ship_id)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)


def test_replace_root_ship_to_struct(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, group_id=eve_grp_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    eve_struct_id = client.mk_eve_struct()
    eve_module_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_ship.remove()
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_fit.set_ship(type_id=eve_struct_id)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
