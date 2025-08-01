from tests import approx, check_no_field
from tests.fw.api import FitStatsOptions, ValOptions


def test_fail_single(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 8000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(9000), approx(8000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(9000)
    assert api_val.details.fighter_bay_volume.max == approx(8000)
    assert api_val.details.fighter_bay_volume.users == {api_fighter.id: approx(9000)}


def test_fail_multiple_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter2_id = client.mk_eve_item(attrs={eve_use_attr_id: 800, eve_count_attr_id: 12})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 15000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter1_id)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter2_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(18600), approx(15000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(18600)
    assert api_val.details.fighter_bay_volume.max == approx(15000)
    assert api_val.details.fighter_bay_volume.users == {api_fighter1.id: approx(9000), api_fighter2.id: approx(9600)}


def test_fail_multiple_struct(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter2_id = client.mk_eve_item(attrs={eve_use_attr_id: 800, eve_count_attr_id: 12})
    eve_struct_id = client.mk_eve_struct(attrs={eve_max_attr_id: 15000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter1_id)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter2_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(18600), approx(15000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(18600)
    assert api_val.details.fighter_bay_volume.max == approx(15000)
    assert api_val.details.fighter_bay_volume.users == {api_fighter1.id: approx(9000), api_fighter2.id: approx(9600)}


def test_equal(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 9000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(9000), approx(9000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter2_id = client.mk_eve_item(attrs={eve_use_attr_id: 800, eve_count_attr_id: 12})
    eve_fighter3_id = client.mk_eve_item(attrs={eve_use_attr_id: -100, eve_count_attr_id: 5})
    eve_fighter4_id = client.mk_eve_item(attrs={eve_use_attr_id: 0, eve_count_attr_id: 20})
    eve_fighter5_id = client.mk_eve_item(attrs={eve_use_attr_id: 0.5, eve_count_attr_id: 3})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 8000})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter1_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(9000), approx(8000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=(True, [api_fighter1.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter2_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(18600), approx(8000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=(True, [api_fighter1.id])))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(18600)
    assert api_val.details.fighter_bay_volume.max == approx(8000)
    assert api_val.details.fighter_bay_volume.users == {api_fighter2.id: approx(9600)}
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=(True, [api_fighter2.id])))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(18600)
    assert api_val.details.fighter_bay_volume.max == approx(8000)
    assert api_val.details.fighter_bay_volume.users == {api_fighter1.id: approx(9000)}
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=(True, [api_fighter1.id, api_fighter2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        fighter_bay_volume=(True, [api_fighter1.id, api_other.id, api_fighter2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter3 = api_fit.add_fighter(type_id=eve_fighter3_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(18100), approx(8000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=(True, [api_fighter1.id, api_fighter2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter3.remove()
    api_fighter4 = api_fit.add_fighter(type_id=eve_fighter4_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(18600), approx(8000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=(True, [api_fighter1.id, api_fighter2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter4.remove()
    api_fighter5 = api_fit.add_fighter(type_id=eve_fighter5_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(18601.5), approx(8000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=(True, [api_fighter1.id, api_fighter2.id])))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(18601.5)
    assert api_val.details.fighter_bay_volume.max == approx(8000)
    assert api_val.details.fighter_bay_volume.users == {api_fighter5.id: approx(1.5)}


def test_changed_count(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 5000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, count=6)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(6000), approx(5000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(6000)
    assert api_val.details.fighter_bay_volume.max == approx(5000)
    assert api_val.details.fighter_bay_volume.users == {api_fighter.id: approx(6000)}
    # Action
    api_fighter.change_fighter(count=5)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(5000), approx(5000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter.change_fighter(count=20)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(20000), approx(5000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(20000)
    assert api_val.details.fighter_bay_volume.max == approx(5000)
    assert api_val.details.fighter_bay_volume.users == {api_fighter.id: approx(20000)}
    # Action
    api_fighter.change_fighter(count=2)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(2000), approx(5000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter.change_fighter(count=None)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(9000), approx(5000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(9000)
    assert api_val.details.fighter_bay_volume.max == approx(5000)
    assert api_val.details.fighter_bay_volume.users == {api_fighter.id: approx(9000)}


def test_modified_count(client, consts):
    # Max fighter squad size is never modified, so the lib just uses unmodified attributes for
    # faster access to the attr value
    eve_skill_id = client.mk_eve_item()
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_count_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_fighter_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 8000})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 12}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    assert api_fighter.update().attrs[eve_count_attr_id].extra == approx(12)
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(9000), approx(8000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(9000)
    assert api_val.details.fighter_bay_volume.max == approx(8000)
    assert api_val.details.fighter_bay_volume.users == {api_fighter.id: approx(9000)}
    # Action
    api_implant.remove()
    # Verification
    assert api_fighter.update().attrs[eve_count_attr_id].extra == approx(9)
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(9000), approx(8000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(9000)
    assert api_val.details.fighter_bay_volume.max == approx(8000)
    assert api_val.details.fighter_bay_volume.users == {api_fighter.id: approx(9000)}


def test_modified_use(client, consts):
    # Fighter volume is never modified, so the lib just uses unmodified attributes for faster access
    # to the attr value
    eve_skill_id = client.mk_eve_item()
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_use_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_fighter_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 8000})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: -50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    assert api_fighter.update().attrs[eve_use_attr_id].extra == approx(500)
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(9000), approx(8000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(9000)
    assert api_val.details.fighter_bay_volume.max == approx(8000)
    assert api_val.details.fighter_bay_volume.users == {api_fighter.id: approx(9000)}
    # Action
    api_implant.remove()
    # Verification
    assert api_fighter.update().attrs[eve_use_attr_id].extra == approx(1000)
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(9000), approx(8000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(9000)
    assert api_val.details.fighter_bay_volume.max == approx(8000)
    assert api_val.details.fighter_bay_volume.users == {api_fighter.id: approx(9000)}


def test_modified_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_fighter_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 8000})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].extra == approx(8000)
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(9000), approx(8000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(9000)
    assert api_val.details.fighter_bay_volume.max == approx(8000)
    assert api_val.details.fighter_bay_volume.users == {api_fighter.id: approx(9000)}
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].extra == approx(12000)
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(9000), approx(12000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_rounding(client, consts):
    # Volume/capacity shouldn't have its sum or individual values rounded
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0.0002, eve_count_attr_id: 10})
    eve_fighter2_id = client.mk_eve_item(attrs={eve_use_attr_id: 0.5227, eve_count_attr_id: 10})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 5.223})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter1_id)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter2_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(5.229), approx(5.223))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(5.229)
    assert api_val.details.fighter_bay_volume.max == approx(5.223)
    assert api_val.details.fighter_bay_volume.users == {api_fighter1.id: approx(0.002), api_fighter2.id: approx(5.227)}


def test_no_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(9000), None)
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(9000)
    assert api_val.details.fighter_bay_volume.max is None
    assert api_val.details.fighter_bay_volume.users == {api_fighter.id: approx(9000)}


def test_not_loaded_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(9000), None)
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(9000)
    assert api_val.details.fighter_bay_volume.max is None
    assert api_val.details.fighter_bay_volume.users == {api_fighter.id: approx(9000)}


def test_not_loaded_user(client, consts):
    # Just check that nothing crashes, not loaded items are not supposed to even be registered
    client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 8000})
    eve_fighter_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(0), approx(8000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_non_positive(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0, eve_count_attr_id: 10})
    eve_fighter2_id = client.mk_eve_item(attrs={eve_use_attr_id: 15, eve_count_attr_id: 10})
    eve_fighter3_id = client.mk_eve_item(attrs={eve_use_attr_id: -1, eve_count_attr_id: 10})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_fighter(type_id=eve_fighter1_id)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter2_id)
    api_fit.add_fighter(type_id=eve_fighter3_id)
    # Verification - items with negative and 0 use are not exposed
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(140), approx(125))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(140)
    assert api_val.details.fighter_bay_volume.max == approx(125)
    assert api_val.details.fighter_bay_volume.users == {api_fighter2.id: approx(150)}


def test_no_value_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter2_id = client.mk_eve_item(attrs={eve_count_attr_id: 9})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 8000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter1_id)
    api_fit.add_fighter(type_id=eve_fighter2_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(9000), approx(8000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(9000)
    assert api_val.details.fighter_bay_volume.max == approx(8000)
    assert api_val.details.fighter_bay_volume.users == {api_fighter1.id: approx(9000)}


def test_no_value_count(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_fighter2_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 8000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter1_id)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter2_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(10000), approx(8000))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(10000)
    assert api_val.details.fighter_bay_volume.max == approx(8000)
    assert api_val.details.fighter_bay_volume.users == {api_fighter1.id: approx(9000), api_fighter2.id: approx(1000)}


def test_no_value_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(9000), approx(0))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is False
    assert api_val.details.fighter_bay_volume.used == approx(9000)
    assert api_val.details.fighter_bay_volume.max == approx(0)
    assert api_val.details.fighter_bay_volume.users == {api_fighter.id: approx(9000)}


def test_criterion_item_kind(client, consts):
    # Validation applies only to fighters
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_capacity)
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_item_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000, eve_count_attr_id: 9})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 900, eve_use_attr_id: 1000})
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_item(attrs={eve_autocharge_attr_id: eve_item_id}, eff_ids=[eve_autocharge_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_item_id)
    api_fit.set_character(type_id=eve_item_id)
    api_fit.add_drone(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit.add_fw_effect(type_id=eve_item_id)
    api_fit.add_implant(type_id=eve_item_id)
    api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.overload, charge_type_id=eve_item_id)
    api_fit.add_rig(type_id=eve_item_id)
    api_fit.add_service(type_id=eve_item_id, state=consts.ApiServiceState.online)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_skill(type_id=eve_item_id, level=5)
    api_fit.set_stance(type_id=eve_item_id)
    api_fit.add_subsystem(type_id=eve_item_id)
    # Verification
    assert len(api_fighter.autocharges) == 1
    api_stats = api_fit.get_stats(options=FitStatsOptions(fighter_bay_volume=True))
    assert api_stats.fighter_bay_volume == (approx(0), approx(900))
    api_val = api_fit.validate(options=ValOptions(fighter_bay_volume=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
