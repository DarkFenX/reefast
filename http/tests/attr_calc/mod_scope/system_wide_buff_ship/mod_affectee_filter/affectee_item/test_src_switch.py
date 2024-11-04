from tests import approx


def test_to_struct(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_d1.mk_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_d2.mk_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_d1.mk_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_d2.mk_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = eve_d1.mk_attr().id
    eve_d2.mk_attr(id_=eve_affectee_attr_id)
    eve_buff_id = eve_d1.mk_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)]).id
    eve_d2.mk_buff(
        id_=eve_buff_id,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_d1.mk_effect(
        id_=consts.EveEffect.mod_titan_effect_generator,
        cat_id=consts.EveEffCat.active)
    eve_d2.mk_effect(
        id_=consts.EveEffect.mod_titan_effect_generator,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = eve_d1.mk_item(
        attrs={consts.EveAttr.warfare_buff_1_id: eve_buff_id, consts.EveAttr.warfare_buff_1_value: 30},
        eff_ids=[consts.EveEffect.mod_titan_effect_generator],
        defeff_id=consts.EveEffect.mod_titan_effect_generator).id
    eve_d2.mk_item(
        id_=eve_sw_effect_id,
        attrs={consts.EveAttr.warfare_buff_1_id: eve_buff_id, consts.EveAttr.warfare_buff_1_value: 30},
        eff_ids=[consts.EveEffect.mod_titan_effect_generator],
        defeff_id=consts.EveEffect.mod_titan_effect_generator)
    eve_root_id = eve_d1.mk_ship(attrs={eve_affectee_attr_id: 200}).id
    eve_d2.mk_struct(id_=eve_root_id, attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_fit = api_sol.create_fit()
    api_root = api_fit.set_ship(type_id=eve_root_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(260)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(260)
