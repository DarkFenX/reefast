from tests import approx


def test_filter(client, consts):
    # Some missile damage modules affect missiles via on-character attribute. Here we make sure it's
    # applied
    eve_attr_bcs_id = client.mk_eve_attr()
    eve_attr_char_id = client.mk_eve_attr(id_=consts.EveAttr.missile_dmg_mult)
    eve_attr_missile_em_id = client.mk_eve_attr(id_=consts.EveAttr.em_dmg)
    eve_attr_missile_therm_id = client.mk_eve_attr(id_=consts.EveAttr.therm_dmg)
    eve_attr_missile_kin_id = client.mk_eve_attr(id_=consts.EveAttr.kin_dmg)
    eve_attr_missile_expl_id = client.mk_eve_attr(id_=consts.EveAttr.expl_dmg)
    eve_effect_online_id = client.mk_eve_online_effect()
    eve_mod_bcs = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.pre_mul,
        affector_attr_id=eve_attr_bcs_id,
        affectee_attr_id=eve_attr_char_id)
    eve_effect_bcs_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod_bcs])
    eve_item_skill1_id = client.mk_eve_item(id_=consts.EveItem.missile_launcher_operation)
    eve_item_skill2_id = client.mk_eve_item()
    eve_item_bcs_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        attrs={eve_attr_bcs_id: 1.1},
        eff_ids=[eve_effect_online_id, eve_effect_bcs_id])
    eve_item_char_id = client.mk_eve_item(grp_id=consts.EveItemGrp.character, attrs={eve_attr_char_id: 1})
    eve_item_launcher_id = client.mk_eve_item()
    eve_item_missile_id = client.mk_eve_item(
        attrs={
            eve_attr_missile_em_id: 50, eve_attr_missile_therm_id: 70,
            eve_attr_missile_kin_id: 80, eve_attr_missile_expl_id: 100},
        srqs={eve_item_skill1_id: 1})
    eve_item_nonmissile_id = client.mk_eve_item(
        attrs={
            eve_attr_missile_em_id: 50, eve_attr_missile_therm_id: 70,
            eve_attr_missile_kin_id: 80, eve_attr_missile_expl_id: 100},
        srqs={eve_item_skill2_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_item_char_id)
    api_fit.add_module(type_id=eve_item_bcs_id, rack=consts.ApiRack.low, state=consts.ApiModuleState.online)
    api_launcher1 = api_fit.add_module(
        type_id=eve_item_launcher_id,
        rack=consts.ApiRack.high,
        charge_type_id=eve_item_missile_id)
    api_launcher2 = api_fit.add_module(
        type_id=eve_item_launcher_id,
        rack=consts.ApiRack.high,
        charge_type_id=eve_item_nonmissile_id)
    api_launcher1.update()
    api_launcher2.update()
    assert api_launcher1.charge.attrs[eve_attr_missile_em_id].dogma == approx(55)
    assert api_launcher1.charge.attrs[eve_attr_missile_therm_id].dogma == approx(77)
    assert api_launcher1.charge.attrs[eve_attr_missile_kin_id].dogma == approx(88)
    assert api_launcher1.charge.attrs[eve_attr_missile_expl_id].dogma == approx(110)
    assert api_launcher2.charge.attrs[eve_attr_missile_em_id].dogma == approx(50)
    assert api_launcher2.charge.attrs[eve_attr_missile_therm_id].dogma == approx(70)
    assert api_launcher2.charge.attrs[eve_attr_missile_kin_id].dogma == approx(80)
    assert api_launcher2.charge.attrs[eve_attr_missile_expl_id].dogma == approx(100)


def test_penalization(client, consts):
    # There are different things which affect missile damage. Some of them are immune to stacking
    # penalties thanks to their carriers being in immune categories, but some are not - like
    # magnetar, wolf-rayet, and plasma storm effect. Here, we check that character modification is
    # not stacking penalized against those.
    eve_item_skill_id = client.mk_eve_item(id_=consts.EveItem.missile_launcher_operation)
    eve_attr_magnetar_id = client.mk_eve_attr()
    eve_attr_char_id = client.mk_eve_attr(id_=consts.EveAttr.missile_dmg_mult)
    eve_attr_missile_em_id = client.mk_eve_attr(id_=consts.EveAttr.em_dmg, stackable=False)
    eve_attr_missile_therm_id = client.mk_eve_attr(id_=consts.EveAttr.therm_dmg, stackable=False)
    eve_attr_missile_kin_id = client.mk_eve_attr(id_=consts.EveAttr.kin_dmg, stackable=False)
    eve_attr_missile_expl_id = client.mk_eve_attr(id_=consts.EveAttr.expl_dmg, stackable=False)
    # Magnetar, wolf-rayet and plasma storm use post multiplication
    eve_mod_magnetar_em = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_item_skill_id,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_attr_magnetar_id,
        affectee_attr_id=eve_attr_missile_em_id)
    eve_mod_magnetar_therm = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_item_skill_id,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_attr_magnetar_id,
        affectee_attr_id=eve_attr_missile_therm_id)
    eve_mod_magnetar_kin = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_item_skill_id,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_attr_magnetar_id,
        affectee_attr_id=eve_attr_missile_kin_id)
    eve_mod_magnetar_expl = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_item_skill_id,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_attr_magnetar_id,
        affectee_attr_id=eve_attr_missile_expl_id)
    eve_effect_magnetar_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.system,
        mod_info=[eve_mod_magnetar_em, eve_mod_magnetar_therm, eve_mod_magnetar_kin, eve_mod_magnetar_expl])
    eve_item_magnetar_id = client.mk_eve_item(
        grp_id=consts.EveItemGrp.effect_beacon,
        cat_id=consts.EveItemCat.celestial,
        attrs={eve_attr_magnetar_id: 1.44},
        eff_ids=[eve_effect_magnetar_id])
    eve_item_char_id = client.mk_eve_item(grp_id=consts.EveItemGrp.character, attrs={eve_attr_char_id: 1.3})
    eve_item_launcher_id = client.mk_eve_item()
    eve_item_missile_id = client.mk_eve_item(
        attrs={
            eve_attr_missile_em_id: 50, eve_attr_missile_therm_id: 70,
            eve_attr_missile_kin_id: 80, eve_attr_missile_expl_id: 100},
        srqs={eve_item_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_char = api_fit.set_character(type_id=eve_item_char_id)
    api_magnetar = api_sol.add_sw_effect(type_id=eve_item_magnetar_id)
    api_launcher = api_fit.add_module(
        type_id=eve_item_launcher_id,
        rack=consts.ApiRack.high,
        charge_type_id=eve_item_missile_id)
    api_launcher.update()
    # Just check values
    assert api_launcher.charge.attrs[eve_attr_missile_em_id].dogma == approx(93.6)
    assert api_launcher.charge.attrs[eve_attr_missile_therm_id].dogma == approx(131.04)
    assert api_launcher.charge.attrs[eve_attr_missile_kin_id].dogma == approx(149.76)
    assert api_launcher.charge.attrs[eve_attr_missile_expl_id].dogma == approx(187.2)
    # In modification info, check that both operators are exposed as post-multiplication (despite
    # on-character effect actually using a bit different operator), and that penalization flag is
    # reported as expected - that on-character effect modification is not getting penalized
    api_em_mods = api_launcher.charge.mods[eve_attr_missile_em_id]
    assert len(api_em_mods) == 2
    api_em_mod1 = api_em_mods.find_by_affector_item(affector_item_id=api_magnetar.id).one()
    assert api_em_mod1.op == consts.ApiModOp.post_mul
    assert api_em_mod1.initial_val == approx(1.44)
    assert api_em_mod1.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_em_mod1.applied_val == approx(1.44)
    api_em_mod2 = api_em_mods.find_by_affector_item(affector_item_id=api_char.id).one()
    assert api_em_mod2.op == consts.ApiModOp.post_mul
    assert api_em_mod2.initial_val == approx(1.3)
    assert api_em_mod2.stacking_mult is None
    assert api_em_mod2.applied_val == approx(1.3)
    api_therm_mods = api_launcher.charge.mods[eve_attr_missile_therm_id]
    assert len(api_therm_mods) == 2
    api_therm_mod1 = api_therm_mods.find_by_affector_item(affector_item_id=api_magnetar.id).one()
    assert api_therm_mod1.op == consts.ApiModOp.post_mul
    assert api_therm_mod1.initial_val == approx(1.44)
    assert api_therm_mod1.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_therm_mod1.applied_val == approx(1.44)
    api_therm_mod2 = api_therm_mods.find_by_affector_item(affector_item_id=api_char.id).one()
    assert api_therm_mod2.op == consts.ApiModOp.post_mul
    assert api_therm_mod2.initial_val == approx(1.3)
    assert api_therm_mod2.stacking_mult is None
    assert api_therm_mod2.applied_val == approx(1.3)
    api_kin_mods = api_launcher.charge.mods[eve_attr_missile_kin_id]
    assert len(api_kin_mods) == 2
    api_kin_mod1 = api_kin_mods.find_by_affector_item(affector_item_id=api_magnetar.id).one()
    assert api_kin_mod1.op == consts.ApiModOp.post_mul
    assert api_kin_mod1.initial_val == approx(1.44)
    assert api_kin_mod1.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_kin_mod1.applied_val == approx(1.44)
    api_kin_mod2 = api_kin_mods.find_by_affector_item(affector_item_id=api_char.id).one()
    assert api_kin_mod2.op == consts.ApiModOp.post_mul
    assert api_kin_mod2.initial_val == approx(1.3)
    assert api_kin_mod2.stacking_mult is None
    assert api_kin_mod2.applied_val == approx(1.3)
    api_expl_mods = api_launcher.charge.mods[eve_attr_missile_expl_id]
    assert len(api_expl_mods) == 2
    api_expl_mod1 = api_expl_mods.find_by_affector_item(affector_item_id=api_magnetar.id).one()
    assert api_expl_mod1.op == consts.ApiModOp.post_mul
    assert api_expl_mod1.initial_val == approx(1.44)
    assert api_expl_mod1.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_expl_mod1.applied_val == approx(1.44)
    api_expl_mod2 = api_expl_mods.find_by_affector_item(affector_item_id=api_char.id).one()
    assert api_expl_mod2.op == consts.ApiModOp.post_mul
    assert api_expl_mod2.initial_val == approx(1.3)
    assert api_expl_mod2.stacking_mult is None
    assert api_expl_mod2.applied_val == approx(1.3)
