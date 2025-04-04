from __future__ import annotations

import typing
from dataclasses import dataclass
from itertools import chain

from tests import effect_dogma_to_api
from tests.fw.util import Default

if typing.TYPE_CHECKING:
    from tests.fw.client import TestClient
    from tests.fw.eve import EveObjects


@dataclass(kw_only=True)
class RahBasicInfo:
    shield_hp_attr_id: int
    armor_hp_attr_id: int
    hull_hp_attr_id: int
    res_max_attr_id: int
    res_em_attr_id: int
    res_therm_attr_id: int
    res_kin_attr_id: int
    res_expl_attr_id: int
    res_shift_attr_id: int
    cycle_time_attr_id: int
    cycle_time_bonus_attr_id: int
    rah_effect_id: int
    heat_effect_id: int

    @property
    def api_rah_effect_id(self) -> str:
        return effect_dogma_to_api(dogma_effect_id=self.rah_effect_id)


def setup_rah_basics(
        *,
        client: TestClient,
        consts,  # noqa: ANN001
        datas: list[EveObjects] | type[Default] = Default,
        attr_shield_hp: int | None | type[Default] = Default,
        attr_armor_hp: int  | None| type[Default] = Default,
        attr_hull_hp: int | None | type[Default] = Default,
        attr_res_em: int | None | type[Default] = Default,
        attr_res_therm: int | None | type[Default] = Default,
        attr_res_kin: int | None | type[Default] = Default,
        attr_res_expl: int | None | type[Default] = Default,
        attr_shift: int | None | type[Default] = Default,
        attr_cycle_time: int | type[Default] = Default,
) -> RahBasicInfo:
    eve_shield_hp_attr_id = _make_optional_attr(
        client=client,
        datas=datas,
        attr_id=consts.EveAttr.shield_capacity if attr_shield_hp is Default else attr_shield_hp)
    eve_armor_hp_attr_id = _make_optional_attr(
        client=client,
        datas=datas,
        attr_id=consts.EveAttr.armor_hp if attr_armor_hp is Default else attr_armor_hp)
    eve_hull_hp_attr_id = _make_optional_attr(
        client=client,
        datas=datas,
        attr_id=consts.EveAttr.hp if attr_hull_hp is Default else attr_hull_hp)
    eve_res_max_attr_id = client.mk_eve_attr(
        datas=datas,
        id_=consts.EveAttr.armor_max_dmg_resonance,
        def_val=1)
    eve_res_em_attr_id = _make_optional_attr(
        client=client,
        datas=datas,
        attr_id=consts.EveAttr.armor_em_dmg_resonance if attr_res_em is Default else attr_res_em,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_therm_attr_id = _make_optional_attr(
        client=client,
        datas=datas,
        attr_id=consts.EveAttr.armor_therm_dmg_resonance if attr_res_therm is Default else attr_res_therm,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_kin_attr_id = _make_optional_attr(
        client=client,
        datas=datas,
        attr_id=consts.EveAttr.armor_kin_dmg_resonance if attr_res_kin is Default else attr_res_kin,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_expl_attr_id = _make_optional_attr(
        client=client,
        datas=datas,
        attr_id=consts.EveAttr.armor_expl_dmg_resonance if attr_res_expl is Default else attr_res_expl,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_shift_attr_id = _make_optional_attr(
        client=client,
        datas=datas,
        attr_id=consts.EveAttr.resist_shift_amount if attr_shift is Default else attr_shift,
        stackable=True)
    eve_cycle_time_attr_id = client.mk_eve_attr(
        datas=datas,
        id_=consts.EveAttr.duration if attr_cycle_time is Default else attr_cycle_time)
    eve_cycle_time_bonus_attr_id = client.mk_eve_attr(datas=datas, id_=consts.EveAttr.overload_self_duration_bonus)
    eve_rah_effect_id = client.mk_eve_effect(
        datas=datas,
        id_=consts.EveEffect.adaptive_armor_hardener,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_heat_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_cycle_time_bonus_attr_id,
        affectee_attr_id=eve_cycle_time_attr_id)
    eve_heat_effect_id = client.mk_eve_effect(
        datas=datas,
        id_=consts.EveEffect.overload_self_duration_bonus,
        cat_id=consts.EveEffCat.overload,
        mod_info=[eve_heat_mod])
    return RahBasicInfo(
        shield_hp_attr_id=eve_shield_hp_attr_id,
        armor_hp_attr_id=eve_armor_hp_attr_id,
        hull_hp_attr_id=eve_hull_hp_attr_id,
        res_max_attr_id=eve_res_max_attr_id,
        res_em_attr_id=eve_res_em_attr_id,
        res_therm_attr_id=eve_res_therm_attr_id,
        res_kin_attr_id=eve_res_kin_attr_id,
        res_expl_attr_id=eve_res_expl_attr_id,
        cycle_time_attr_id=eve_cycle_time_attr_id,
        cycle_time_bonus_attr_id=eve_cycle_time_bonus_attr_id,
        res_shift_attr_id=eve_res_shift_attr_id,
        rah_effect_id=eve_rah_effect_id,
        heat_effect_id=eve_heat_effect_id)


def _make_optional_attr(
        *,
        client: TestClient,
        datas: list[EveObjects] | type[Default] = Default,
        attr_id: int | None,
        stackable: bool | type[Default] = Default,
        max_attr_id: int | type[Default] = Default,
) -> int | None:
    if attr_id is None:
        return None
    return client.mk_eve_attr(datas=datas, id_=attr_id, stackable=stackable, max_attr_id=max_attr_id)


def make_eve_rah(
        *,
        client: TestClient,
        datas: list[EveObjects] | type[Default] = Default,
        basic_info: RahBasicInfo,
        id_: int | type[Default] = Default,
        grp_id: int | type[Default] = Default,
        resos: tuple[float, float, float, float],
        shift_amount: float = 6,
        cycle_time: float = 10000,
        heat_cycle_mod: float = -15,
) -> int:
    return client.mk_eve_item(
        datas=datas,
        id_=id_,
        grp_id=grp_id,
        attrs={
            k: v for k, v in zip(
                (basic_info.res_em_attr_id,
                 basic_info.res_therm_attr_id,
                 basic_info.res_kin_attr_id,
                 basic_info.res_expl_attr_id,
                 basic_info.res_shift_attr_id,
                 basic_info.cycle_time_attr_id,
                 basic_info.cycle_time_bonus_attr_id),
                (*resos,
                 shift_amount,
                 cycle_time,
                 heat_cycle_mod),
                strict=True)
            if k is not None},
        eff_ids=[basic_info.rah_effect_id, basic_info.heat_effect_id],
        defeff_id=basic_info.rah_effect_id)


def make_eve_ship(
        *,
        client: TestClient,
        datas: list[EveObjects] | type[Default] = Default,
        basic_info: RahBasicInfo,
        id_: int | type[Default] = Default,
        resos: tuple[float, float, float, float],
        hps: tuple[float, float, float] | type[Default] = Default,
) -> int:
    if hps is Default:
        hps = (0, 0, 0)
    return client.mk_eve_ship(datas=datas, id_=id_, attrs={
        k: v for k, v in chain(
            zip(
                (basic_info.res_em_attr_id,
                 basic_info.res_therm_attr_id,
                 basic_info.res_kin_attr_id,
                 basic_info.res_expl_attr_id),
                resos,
                strict=True),
            zip(
                (basic_info.shield_hp_attr_id,
                 basic_info.armor_hp_attr_id,
                 basic_info.hull_hp_attr_id),
                hps,
                strict=True))
        if k is not None})
