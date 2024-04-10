from pytest import approx


def test_affected_removal(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.char,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 100})
    eve_char_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_char(type_id=eve_char_item.id)
    api_src_item = api_fit.add_rig(type_id=eve_src_item.id)
    api_tgt_item = api_fit.add_implant(type_id=eve_tgt_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(120)
    api_src_item.remove()
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_affected_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_grp = client.mk_eve_item_group()
    eve_src_attr = client.mk_eve_attr()
    eve_mid_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_mid_attr.id)
    eve_src_effect = client.mk_eve_effect(mod_info=[eve_src_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 2}, eff_ids=[eve_src_effect.id])
    eve_mid_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.char,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_mid_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_mid_effect = client.mk_eve_effect(mod_info=[eve_mid_mod])
    eve_mid_item = client.mk_eve_item(attrs={eve_mid_attr.id: 20}, eff_ids=[eve_mid_effect.id])
    eve_tgt_item = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 100})
    eve_char_item = client.mk_eve_item()
    eve_ship_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_char(type_id=eve_char_item.id)
    api_tgt_item = api_fit.add_implant(type_id=eve_tgt_item.id)
    api_fit.set_ship(type_id=eve_ship_item.id)
    api_fit.add_rig(type_id=eve_mid_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(120)
    api_src_item = api_fit.add_rig(type_id=eve_src_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(140)
    api_src_item.remove()
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(120)


def test_unaffected_other_domain(client, consts):
    # Check that entities from other domains are not affected
    eve_grp = client.mk_eve_item_group()
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.char,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 100})
    eve_char_item = client.mk_eve_item()
    eve_ship_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_char(type_id=eve_char_item.id)
    api_fit.set_ship(type_id=eve_ship_item.id)
    api_fit.add_rig(type_id=eve_src_item.id)
    api_tgt_item = api_fit.add_rig(type_id=eve_tgt_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_unaffected_other_group(client, consts):
    # Check that entities belonging to other item groups are not affected
    eve_grp1 = client.mk_eve_item_group()
    eve_grp2 = client.mk_eve_item_group()
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.char,
        grp=eve_grp1.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 100}, grp_id=eve_grp2.id)
    eve_char_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_char(type_id=eve_char_item.id)
    api_fit.add_rig(type_id=eve_src_item.id)
    api_tgt_item = api_fit.add_implant(type_id=eve_tgt_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_unaffected_other_fit(client, consts):
    # Check that local modifications are not carried over to another fit
    eve_grp = client.mk_eve_item_group()
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.char,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 100})
    eve_char_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit1 = api_ss.create_fit()
    api_fit2 = api_ss.create_fit()
    api_fit1.set_char(type_id=eve_char_item.id)
    api_fit2.set_char(type_id=eve_char_item.id)
    api_fit1.add_rig(type_id=eve_src_item.id)
    api_tgt_item = api_fit2.add_implant(type_id=eve_tgt_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_replace_parent(client, consts):
    # Modifiers which target items on character location shouldn't apply when character isn't set
    eve_grp = client.mk_eve_item_group()
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.char,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 100})
    eve_char_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.add_rig(type_id=eve_src_item.id)
    api_char_item = api_fit.set_char(type_id=eve_char_item.id)
    api_tgt_item = api_fit.add_implant(type_id=eve_tgt_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(120)
    api_char_item.remove()
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(100)
    api_fit.set_char(type_id=eve_char_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(120)
