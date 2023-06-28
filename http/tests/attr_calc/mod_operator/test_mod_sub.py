from pytest import approx


def get_dogma_value(client, consts, stackable):
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr(stackable=stackable)
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.ship,
        op=consts.ModOp.mod_sub,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_src1 = client.mk_eve_item(attrs={eve_src_attr.id: -50}, eff_ids=[eve_effect.id])
    eve_item_src2 = client.mk_eve_item(attrs={eve_src_attr.id: 23}, eff_ids=[eve_effect.id])
    eve_item_src3 = client.mk_eve_item(attrs={eve_src_attr.id: 53.08}, eff_ids=[eve_effect.id])
    eve_item_tgt = client.mk_eve_item(attrs={eve_tgt_attr.id: 70})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.add_rig(type_id=eve_item_src1.id)
    api_fit.add_rig(type_id=eve_item_src2.id)
    api_fit.add_rig(type_id=eve_item_src3.id)
    api_item_tgt = api_fit.set_ship(type_id=eve_item_tgt.id)
    return api_item_tgt.update().attrs[eve_tgt_attr.id].dogma


def test_non_penalized(client, consts):
    value = get_dogma_value(client, consts, stackable=True)
    assert value == approx(43.92)


def test_penalized(client, consts):
    value = get_dogma_value(client, consts, stackable=False)
    assert value == approx(43.92)
