from tests import approx


def test_replace_proj(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_ships, cat_id=consts.EveEffCat.active)
    eve_proj_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 30},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    eve_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship1 = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship1.id])
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(260)
    api_ship1.remove()
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_ship2 = api_fit.set_ship(type_id=eve_ship_id)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_proj_effect.change_proj_effect(add_projs=[api_ship2.id])
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(260)
