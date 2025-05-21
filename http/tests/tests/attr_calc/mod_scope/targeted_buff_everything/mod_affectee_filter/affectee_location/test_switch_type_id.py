from tests import approx, check_no_field


def test_affectee_rig(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_rig1_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200})
    eve_rig2_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_rig3_id = client.alloc_item_id()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig1_id)
    api_module.change_module(add_projs=[api_ship.id])
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    # Action
    api_rig.change_rig(type_id=eve_rig2_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(45)
    # Action
    api_rig.change_rig(type_id=eve_rig3_id)
    # Verification
    api_rig.update()
    with check_no_field():
        api_rig.attrs  # noqa: B018
    # Action
    api_rig.change_rig(type_id=eve_rig1_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(90)
