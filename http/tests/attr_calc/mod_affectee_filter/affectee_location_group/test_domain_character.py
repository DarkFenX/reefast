from pytest import approx


def test_affected(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.loc_grp,
        dom=consts.ModDom.char,
        grp=eve_grp.id,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_src_item = api_fit.add_rig(type_id=eve_src_item.id)
    api_tgt_item = api_fit.add_implant(type_id=eve_tgt_item.id)
    value = api_tgt_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(120)
    api_src_item.remove()
    value = api_tgt_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(100)


def test_other_location(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.loc_grp,
        dom=consts.ModDom.char,
        grp=eve_grp.id,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.add_rig(type_id=eve_src_item.id)
    api_tgt_item = api_fit.add_rig(type_id=eve_tgt_item.id)
    value = api_tgt_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(100)


def test_other_group(client, consts):
    eve_grp1 = client.mk_eve_item_group()
    eve_grp2 = client.mk_eve_item_group()
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.loc_grp,
        dom=consts.ModDom.char,
        grp=eve_grp1.id,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 100}, grp_id=eve_grp2.id)
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.add_rig(type_id=eve_src_item.id)
    api_tgt_item = api_fit.add_implant(type_id=eve_tgt_item.id)
    value = api_tgt_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(100)
