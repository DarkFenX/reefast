from tests import approx


def test_bubble_sig_local(client, consts):
    # Bubble blows sig of the ship it's on
    eve_sig_attr = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_sig_bonus_attr = client.mk_eve_attr(id_=consts.EveAttr.sig_radius_bonus)
    eve_wdfg_effect = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg = client.mk_eve_item(
        attrs={eve_sig_bonus_attr.id: 50},
        eff_ids=[eve_wdfg_effect.id],
        defeff_id=eve_wdfg_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_sig_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_mod(type_id=eve_wdfg.id, state=consts.ApiState.active)
    # Verification
    assert api_ship.update().attrs[eve_sig_attr.id].dogma == approx(150)


def test_bubble_sig_projected(client, consts):
    # Bubble doesn't blow sig of the ship it's projected to
    eve_sig_attr = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_sig_bonus_attr = client.mk_eve_attr(id_=consts.EveAttr.sig_radius_bonus)
    eve_wdfg_effect = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg = client.mk_eve_item(
        attrs={eve_sig_bonus_attr.id: 50},
        eff_ids=[eve_wdfg_effect.id],
        defeff_id=eve_wdfg_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_sig_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_mod(type_id=eve_wdfg.id, state=consts.ApiState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_wdfg.change_mod(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_sig_attr.id].dogma == approx(100)


def test_bubble_assist_local(client, consts):
    # Bubble disables assistance on the ship it's running on
    eve_assist_attr = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_wdfg_effect = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg = client.mk_eve_item(
        attrs={eve_assist_attr.id: 1},
        eff_ids=[eve_wdfg_effect.id],
        defeff_id=eve_wdfg_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_assist_attr.id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_mod(type_id=eve_wdfg.id, state=consts.ApiState.active)
    # Verification
    assert api_ship.update().attrs[eve_assist_attr.id].dogma == approx(1)


def test_bubble_assist_projected(client, consts):
    # Bubble doesn't disable assistance on any other ships
    eve_assist_attr = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_wdfg_effect = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg = client.mk_eve_item(
        attrs={eve_assist_attr.id: 1},
        eff_ids=[eve_wdfg_effect.id],
        defeff_id=eve_wdfg_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_assist_attr.id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_mod(type_id=eve_wdfg.id, state=consts.ApiState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_wdfg.change_mod(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_assist_attr.id].dogma == approx(0)


def test_warp_scram_status_dscript(client, consts):
    # Disruption script disables warp for target it's projected on
    eve_str_attr = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_strength, def_val=0)
    eve_status_attr = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status, def_val=0)
    eve_wdfg_effect = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg = client.mk_eve_item(
        attrs={eve_str_attr.id: 100},
        eff_ids=[eve_wdfg_effect.id],
        defeff_id=eve_wdfg_effect.id)
    eve_script_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_str_attr.id,
        affectee_attr_id=eve_status_attr.id)
    eve_script_effect = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_disruption_script,
        cat_id=consts.EveEffCat.target,
        mod_info=[eve_script_mod])
    eve_script = client.mk_eve_item(
        eff_ids=[eve_script_effect.id],
        defeff_id=eve_script_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_status_attr.id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_mod(type_id=eve_wdfg.id, state=consts.ApiState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_wdfg.change_mod(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_status_attr.id].dogma == approx(0)
    # Action
    api_wdfg.change_mod(charge=eve_script.id)
    # Verification
    assert api_ship.update().attrs[eve_status_attr.id].dogma == approx(100)
    # Action
    api_wdfg.change_mod(charge=None)
    # Verification
    assert api_ship.update().attrs[eve_status_attr.id].dogma == approx(0)


def test_warp_scram_status_sscript(client, consts):
    # Scrambling script disables warp for target it's projected on
    eve_str_attr = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_strength, def_val=0)
    eve_status_attr = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status, def_val=0)
    eve_wdfg_effect = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg = client.mk_eve_item(
        attrs={eve_str_attr.id: 100},
        eff_ids=[eve_wdfg_effect.id],
        defeff_id=eve_wdfg_effect.id)
    eve_script_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_str_attr.id,
        affectee_attr_id=eve_status_attr.id)
    eve_script_effect = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_scrambling_script,
        cat_id=consts.EveEffCat.target,
        mod_info=[eve_script_mod])
    eve_script = client.mk_eve_item(
        eff_ids=[eve_script_effect.id],
        defeff_id=eve_script_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_status_attr.id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_mod(type_id=eve_wdfg.id, state=consts.ApiState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_wdfg.change_mod(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_status_attr.id].dogma == approx(0)
    # Action
    api_wdfg.change_mod(charge=eve_script.id)
    # Verification
    assert api_ship.update().attrs[eve_status_attr.id].dogma == approx(100)
    # Action
    api_wdfg.change_mod(charge=None)
    # Verification
    assert api_ship.update().attrs[eve_status_attr.id].dogma == approx(0)


def test_gate_scram_status_dscript(client, consts):
    # Disruption script disables gate jumps for target capitals it's projected onto
    eve_str_attr = client.mk_eve_attr(id_=consts.EveAttr.gate_scramble_strength, def_val=1)
    eve_status_attr = client.mk_eve_attr(id_=consts.EveAttr.gate_scramble_status, def_val=0)
    eve_wdfg_effect = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg = client.mk_eve_item(eff_ids=[eve_wdfg_effect.id], defeff_id=eve_wdfg_effect.id)
    eve_script_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_str_attr.id,
        affectee_attr_id=eve_status_attr.id)
    eve_script_effect = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_disruption_script,
        cat_id=consts.EveEffCat.target,
        mod_info=[eve_script_mod])
    eve_script = client.mk_eve_item(
        eff_ids=[eve_script_effect.id],
        defeff_id=eve_script_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_status_attr.id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_mod(type_id=eve_wdfg.id, state=consts.ApiState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_wdfg.change_mod(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_status_attr.id].dogma == approx(0)
    # Action
    api_wdfg.change_mod(charge=eve_script.id)
    # Verification
    assert api_ship.update().attrs[eve_status_attr.id].dogma == approx(1)
    # Action
    api_wdfg.change_mod(charge=None)
    # Verification
    assert api_ship.update().attrs[eve_status_attr.id].dogma == approx(0)


def test_gate_scram_status_sscript(client, consts):
    # Scrambling script disables gate jumps for target capitals it's projected onto
    eve_str_attr = client.mk_eve_attr(id_=consts.EveAttr.gate_scramble_strength, def_val=1)
    eve_status_attr = client.mk_eve_attr(id_=consts.EveAttr.gate_scramble_status, def_val=0)
    eve_wdfg_effect = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg = client.mk_eve_item(eff_ids=[eve_wdfg_effect.id], defeff_id=eve_wdfg_effect.id)
    eve_script_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_str_attr.id,
        affectee_attr_id=eve_status_attr.id)
    eve_script_effect = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_scrambling_script,
        cat_id=consts.EveEffCat.target,
        mod_info=[eve_script_mod])
    eve_script = client.mk_eve_item(
        eff_ids=[eve_script_effect.id],
        defeff_id=eve_script_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_status_attr.id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_mod(type_id=eve_wdfg.id, state=consts.ApiState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_wdfg.change_mod(add_projs=[api_ship.id])
    # Verification
    assert api_ship.update().attrs[eve_status_attr.id].dogma == approx(0)
    # Action
    api_wdfg.change_mod(charge=eve_script.id)
    # Verification
    assert api_ship.update().attrs[eve_status_attr.id].dogma == approx(1)
    # Action
    api_wdfg.change_mod(charge=None)
    # Verification
    assert api_ship.update().attrs[eve_status_attr.id].dogma == approx(0)


def test_mwd_block_dscript(client, consts):
    # Disruption script doesn't disable micro warp drives
    eve_skill = client.mk_eve_item(id_=consts.EveItem.high_speed_maneuvering)
    eve_str_attr = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_wdfg_effect = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg = client.mk_eve_item(
        attrs={eve_str_attr.id: 1},
        eff_ids=[eve_wdfg_effect.id],
        defeff_id=eve_wdfg_effect.id)
    eve_script_effect = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_disruption_script,
        cat_id=consts.EveEffCat.target)
    eve_script = client.mk_eve_item(eff_ids=[eve_script_effect.id], defeff_id=eve_script_effect.id)
    eve_ship = client.mk_eve_ship()
    eve_mwd = client.mk_eve_item(attrs={eve_block_attr.id: 0}, srqs={eve_skill.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_mod(type_id=eve_wdfg.id, state=consts.ApiState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_mwd = api_affectee_fit.add_mod(type_id=eve_mwd.id)
    api_wdfg.change_mod(add_projs=[api_ship.id])
    # Verification
    assert api_mwd.update().attrs[eve_block_attr.id].dogma == approx(0)
    # Action
    api_wdfg.change_mod(charge=eve_script.id)
    # Verification
    assert api_mwd.update().attrs[eve_block_attr.id].dogma == approx(0)
    # Action
    api_wdfg.change_mod(charge=None)
    # Verification
    assert api_mwd.update().attrs[eve_block_attr.id].dogma == approx(0)


def test_mwd_block_sscript(client, consts):
    # Scrambling script disables micro warp drives
    eve_skill = client.mk_eve_item(id_=consts.EveItem.high_speed_maneuvering)
    eve_str_attr = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_wdfg_effect = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg = client.mk_eve_item(
        attrs={eve_str_attr.id: 1},
        eff_ids=[eve_wdfg_effect.id],
        defeff_id=eve_wdfg_effect.id)
    eve_script_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_srq,
        dom=consts.EveModDom.tgt,
        srq=eve_skill.id,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_str_attr.id,
        affectee_attr_id=eve_block_attr.id)
    eve_script_effect = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_scrambling_script,
        cat_id=consts.EveEffCat.target,
        mod_info=[eve_script_mod])
    eve_script = client.mk_eve_item(eff_ids=[eve_script_effect.id], defeff_id=eve_script_effect.id)
    eve_ship = client.mk_eve_ship()
    eve_mwd = client.mk_eve_item(attrs={eve_block_attr.id: 0}, srqs={eve_skill.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_mod(type_id=eve_wdfg.id, state=consts.ApiState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_mwd = api_affectee_fit.add_mod(type_id=eve_mwd.id)
    api_wdfg.change_mod(add_projs=[api_ship.id])
    # Verification
    assert api_mwd.update().attrs[eve_block_attr.id].dogma == approx(0)
    # Action
    api_wdfg.change_mod(charge=eve_script.id)
    # Verification
    assert api_mwd.update().attrs[eve_block_attr.id].dogma == approx(1)
    # Action
    api_wdfg.change_mod(charge=None)
    # Verification
    assert api_mwd.update().attrs[eve_block_attr.id].dogma == approx(0)


def test_mjd_block_dscript(client, consts):
    # Disruption script disables micro jump drives
    eve_skill_sub = client.mk_eve_item(id_=consts.EveItem.micro_jump_drive_operation)
    # Capital MJFG doesn't use regular skill, so test capital skill separately
    eve_skill_cap = client.mk_eve_item(id_=consts.EveItem.capital_micro_jump_drive_operation)
    eve_str_attr = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_wdfg_effect = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg = client.mk_eve_item(
        attrs={eve_str_attr.id: 1},
        eff_ids=[eve_wdfg_effect.id],
        defeff_id=eve_wdfg_effect.id)
    eve_script_effect = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_disruption_script,
        cat_id=consts.EveEffCat.target)
    eve_script = client.mk_eve_item(eff_ids=[eve_script_effect.id], defeff_id=eve_script_effect.id)
    eve_ship = client.mk_eve_ship()
    eve_mjd_sub = client.mk_eve_item(attrs={eve_block_attr.id: 0}, srqs={eve_skill_sub.id: 1})
    eve_mjd_cap = client.mk_eve_item(attrs={eve_block_attr.id: 0}, srqs={eve_skill_cap.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_mod(type_id=eve_wdfg.id, state=consts.ApiState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_mjd_sub = api_affectee_fit.add_mod(type_id=eve_mjd_sub.id)
    api_mjd_cap = api_affectee_fit.add_mod(type_id=eve_mjd_cap.id)
    api_wdfg.change_mod(add_projs=[api_ship.id])
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr.id].dogma == approx(0)
    assert api_mjd_cap.update().attrs[eve_block_attr.id].dogma == approx(0)
    # Action
    api_wdfg.change_mod(charge=eve_script.id)
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr.id].dogma == approx(1)
    assert api_mjd_cap.update().attrs[eve_block_attr.id].dogma == approx(1)
    # Action
    api_wdfg.change_mod(charge=None)
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr.id].dogma == approx(0)
    assert api_mjd_cap.update().attrs[eve_block_attr.id].dogma == approx(0)


def test_mjd_block_sscript(client, consts):
    # Scrambling script disables micro jump drives
    eve_skill_sub = client.mk_eve_item(id_=consts.EveItem.micro_jump_drive_operation)
    # Capital MJFG doesn't use regular skill, so test capital skill separately
    eve_skill_cap = client.mk_eve_item(id_=consts.EveItem.capital_micro_jump_drive_operation)
    eve_str_attr = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked_strength, def_val=0)
    eve_block_attr = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_wdfg_effect = client.mk_eve_effect(id_=consts.EveEffect.warp_disrupt_sphere, cat_id=consts.EveEffCat.active)
    eve_wdfg = client.mk_eve_item(
        attrs={eve_str_attr.id: 1},
        eff_ids=[eve_wdfg_effect.id],
        defeff_id=eve_wdfg_effect.id)
    eve_script_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_srq,
        dom=consts.EveModDom.tgt,
        srq=eve_skill_sub.id,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_str_attr.id,
        affectee_attr_id=eve_block_attr.id)
    eve_script_effect = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_focused_warp_scrambling_script,
        cat_id=consts.EveEffCat.target,
        mod_info=[eve_script_mod])
    eve_script = client.mk_eve_item(eff_ids=[eve_script_effect.id], defeff_id=eve_script_effect.id)
    eve_ship = client.mk_eve_ship()
    eve_mjd_sub = client.mk_eve_item(attrs={eve_block_attr.id: 0}, srqs={eve_skill_sub.id: 1})
    eve_mjd_cap = client.mk_eve_item(attrs={eve_block_attr.id: 0}, srqs={eve_skill_cap.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_wdfg = api_affector_fit.add_mod(type_id=eve_wdfg.id, state=consts.ApiState.active)
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_mjd_sub = api_affectee_fit.add_mod(type_id=eve_mjd_sub.id)
    api_mjd_cap = api_affectee_fit.add_mod(type_id=eve_mjd_cap.id)
    api_wdfg.change_mod(add_projs=[api_ship.id])
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr.id].dogma == approx(0)
    assert api_mjd_cap.update().attrs[eve_block_attr.id].dogma == approx(0)
    # Action
    api_wdfg.change_mod(charge=eve_script.id)
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr.id].dogma == approx(1)
    assert api_mjd_cap.update().attrs[eve_block_attr.id].dogma == approx(1)
    # Action
    api_wdfg.change_mod(charge=None)
    # Verification
    assert api_mjd_sub.update().attrs[eve_block_attr.id].dogma == approx(0)
    assert api_mjd_cap.update().attrs[eve_block_attr.id].dogma == approx(0)
