from tests import approx


def test_unaffected_self(client, consts):
    # EVE does not use effects with "itemID" domain with owner modifiable filter, so it's an
    # undefined behavior. Reefast just discards this modification as invalid
    eve_skill = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.item,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_item = client.mk_eve_item(
        attrs={eve_affector_attr.id: 20, eve_affectee_attr.id: 100},
        eff_ids=[eve_effect.id],
        srqs={eve_skill.id: 1})
    eve_char_item = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_char(type_id=eve_char_item.id)
    api_item = api_fit.add_fw_effect(type_id=eve_item.id)
    assert api_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_root(client, consts):
    # EVE does not use effects with "itemID" domain with owner modifiable filter, so it's an
    # undefined behavior. Reefast just discards this modification as invalid
    eve_skill = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.item,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100}, srqs={eve_skill.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fw_effect(type_id=eve_affector_item.id)
    api_affectee_item = api_fit.set_char(type_id=eve_affectee_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_child(client, consts):
    # EVE does not use effects with "itemID" domain with owner modifiable filter, so it's an
    # undefined behavior. Reefast just discards this modification as invalid
    eve_skill = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.item,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100}, srqs={eve_skill.id: 1})
    eve_char_item = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_char(type_id=eve_char_item.id)
    api_fit.add_fw_effect(type_id=eve_affector_item.id)
    api_affectee_item = api_fit.add_drone(type_id=eve_affectee_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)
