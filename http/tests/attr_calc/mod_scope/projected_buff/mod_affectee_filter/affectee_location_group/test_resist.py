from pytest import approx


def test_resisted_value_change(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_resist_attr = client.mk_eve_attr()
    eve_boost_attr = client.mk_eve_attr()
    eve_proj_effect_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, group_id=eve_grp.id)])
    eve_proj_effect_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active,
        resist_attr_id=eve_resist_attr.id)
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_proj_effect_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_proj_effect_effect.id], defeff_id=eve_proj_effect_effect.id)
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_boost_attr.id,
        affectee_attr_id=eve_resist_attr.id)
    eve_rig_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig = client.mk_eve_item(attrs={eve_boost_attr.id: -25}, eff_ids=[eve_rig_effect.id])
    eve_struct = client.mk_eve_struct(attrs={eve_resist_attr.id: 0.4})
    eve_module = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_ship(type_id=eve_struct.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_projs=[api_struct.id])
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(19.5)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(16.5)
    api_rig.remove()
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(19.5)
