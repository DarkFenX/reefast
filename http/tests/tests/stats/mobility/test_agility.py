from tests import approx
from tests.fw.api import StatsOptions


def test_ship_modified_mass(client, consts):
    eve_agility_attr_id = client.mk_eve_attr(id_=consts.EveAttr.agility)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_mass_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 500000}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_agility_attr_id: 3.2, eve_mass_attr_id: 1050000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(agility=True, align_time=True))
    assert api_stats.agility == approx(4.657949)
    assert api_stats.align_time == 5
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(agility=True, align_time=True))
    assert api_stats.agility == approx(6.87602)
    assert api_stats.align_time == 7
    # Action
    api_rig.remove()
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(agility=True, align_time=True))
    assert api_stats.agility == approx(4.657949)
    assert api_stats.align_time == 5


def test_ship_modified_agility(client, consts):
    eve_agility_attr_id = client.mk_eve_attr(id_=consts.EveAttr.agility)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_mass_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: -20}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_agility_attr_id: 3.2, eve_mass_attr_id: 1050000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(agility=True, align_time=True))
    assert api_stats.agility == approx(4.657949)
    assert api_stats.align_time == 5
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(agility=True, align_time=True))
    assert api_stats.agility == approx(3.726359)
    assert api_stats.align_time == 4
    # Action
    api_rig.remove()
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(agility=True, align_time=True))
    assert api_stats.agility == approx(4.657949)
    assert api_stats.align_time == 5


def test_ship_zero_value_mass(client, consts):
    eve_agility_attr_id = client.mk_eve_attr(id_=consts.EveAttr.agility)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_ship_id = client.mk_eve_ship(attrs={eve_agility_attr_id: 3.2, eve_mass_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(agility=True, align_time=True))
    assert api_stats.agility is None
    assert api_stats.align_time is None


def test_ship_zero_value_agility(client, consts):
    eve_agility_attr_id = client.mk_eve_attr(id_=consts.EveAttr.agility)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_ship_id = client.mk_eve_ship(attrs={eve_agility_attr_id: 0, eve_mass_attr_id: 1050000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(agility=True, align_time=True))
    assert api_stats.agility is None
    assert api_stats.align_time is None


def test_ship_no_value_mass(client, consts):
    eve_agility_attr_id = client.mk_eve_attr(id_=consts.EveAttr.agility)
    client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_ship_id = client.mk_eve_ship(attrs={eve_agility_attr_id: 3.2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(agility=True, align_time=True))
    assert api_stats.agility is None
    assert api_stats.align_time is None


def test_ship_no_value_agility(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.agility)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1050000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(agility=True, align_time=True))
    assert api_stats.agility is None
    assert api_stats.align_time is None


def test_ship_no_ship(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.agility)
    client.mk_eve_attr(id_=consts.EveAttr.mass)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(agility=True, align_time=True))
    assert api_stats.agility is None
    assert api_stats.align_time is None


def test_ship_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.agility)
    client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(agility=True, align_time=True))
    assert api_stats.agility is None
    assert api_stats.align_time is None
