import inspect

import requests

from .data import TestObjects
from ..consts import EffectCategory
from ..util import Absent, Default


data_id = 10000000


def frame_to_primitive(frame, ignore_local_context=False):
    if ignore_local_context:
        return (
            frame.filename,
            frame.function)
    else:
        pos = frame.positions
        return (
            frame.filename,
            frame.lineno,
            frame.function,
            pos.lineno,
            pos.end_lineno,
            pos.col_offset,
            pos.end_col_offset)


def get_stack_key():
    stack = inspect.stack(context=0)
    # Filter out stack entries for entities from client file
    stack = [f for f in stack if f.filename != __file__]
    # For method which tried to retrieve data, ignore all its local context,
    # to refer to the same data on different calls
    key = [frame_to_primitive(stack[0], ignore_local_context=True)]
    key += [frame_to_primitive(f) for f in stack[1:]]
    return tuple(key)


class TestClient:

    def __init__(self, data_server):
        self.__datas = {}
        self.__data_server = data_server
        self.__stack_alias_map = {}
        self.__session = requests.Session()

    def mk_data(self):
        global data_id
        alias = str(data_id)
        data = self.__datas[alias] = TestObjects(alias)
        data_id += 1
        return data

    @property
    def __default_data(self):
        key = get_stack_key()
        if key in self.__stack_alias_map:
            alias = self.__stack_alias_map[key]
            return self.__datas[alias]
        data = self.mk_data()
        self.__stack_alias_map[key] = data.alias
        return data

    def mk_item(
            self,
            data=Default,
            id=Default,
            grp_id=Default,
            cat_id=Default,
            attrs=Default,
            eff_ids=Default,
            defeff_id=None,
            srqs=Default,
    ):
        if data is Default:
            data = self.__default_data
        return data.mk_item(
            id=id,
            group_id=grp_id,
            category_id=cat_id,
            attributes={} if attrs is Default else attrs,
            effect_ids=[] if eff_ids is Default else eff_ids,
            default_effect_id=defeff_id,
            skill_reqs={} if srqs is Default else srqs)

    def mk_attr(
            self,
            data=Default,
            id=Default,
            stackable=1,
            high_is_good=1,
            def_val=0.0,
            max_attr_id=Absent,
    ):
        if data is Default:
            data = self.__default_data
        return data.mk_attr(
            id=id,
            stackable=stackable,
            high_is_good=high_is_good,
            default_value=def_val,
            max_attribute_id=max_attr_id)

    def mk_effect(
            self,
            data=Default,
            id=Default,
            cat_id=EffectCategory.passive,
            is_assistance=False,
            is_offensive=False,
            discharge_attr_id=Absent,
            duration_attr_id=Absent,
            range_attr_id=Absent,
            falloff_attr_id=Absent,
            tracking_attr_id=Absent,
            chance_attr_id=Absent,
            resist_attr_id=Absent,
            mod_info=Absent,
    ):
        if data is Default:
            data = self.__default_data
        return data.mk_effect(
            id=id,
            category_id=cat_id,
            is_assistance=is_assistance,
            is_offensive=is_offensive,
            discharge_attribute_id=discharge_attr_id,
            duration_attribute_id=duration_attr_id,
            range_attribute_id=range_attr_id,
            falloff_attribute_id=falloff_attr_id,
            tracking_attribute_id=tracking_attr_id,
            usage_chance_attribute_id=chance_attr_id,
            resist_attribute_id=resist_attr_id,
            modifier_info=mod_info)

    def mk_buff(
            self,
            data=Default,
            id=Default,
            aggr_mode=Default,
            op=Default,
            item_mods=Default,
            loc_mods=Default,
            loc_grp_mods=Default,
            loc_srq_mods=Default,
    ):
        if data is Default:
            data = self.__default_data
        return data.mk_buff(
            id=id,
            aggregate_mode=aggr_mode,
            operation_name=op,
            item_modifiers=item_mods,
            location_modifiers=loc_mods,
            location_group_modifiers=loc_grp_mods,
            location_skillreq_modifiers=loc_srq_mods)

    def create_source_request(self, data=Default):
        if data is Default:
            data = self.__default_data
        req = requests.Request('POST', f'http://localhost:8000/source/{data.alias}', json={
            'data_version': '1',
            'data_base_url': f'http://localhost:{self.__data_server.port}/'})
        return req

    def create_source(self, data=Default):
        if data is Default:
            data = self.__default_data
        # Set up server with local data
        string_data = data.render()
        self.__data_server.expect_request('/fsd_binary/types.json').respond_with_data(string_data.types)
        self.__data_server.expect_request('/fsd_binary/groups.json').respond_with_data(string_data.groups)
        self.__data_server.expect_request('/fsd_binary/dogmaattributes.json').respond_with_data(string_data.dogmaattributes)
        self.__data_server.expect_request('/fsd_binary/typedogma.json').respond_with_data(string_data.typedogma)
        self.__data_server.expect_request('/fsd_binary/dogmaeffects.json').respond_with_data(string_data.dogmaeffects)
        self.__data_server.expect_request('/fsd_lite/fighterabilities.json').respond_with_data(string_data.fighterabilities)
        self.__data_server.expect_request('/fsd_lite/fighterabilitiesbytype.json').respond_with_data(string_data.fighterabilitiesbytype)
        self.__data_server.expect_request('/fsd_lite/dbuffcollections.json').respond_with_data(string_data.dbuffcollections)
        self.__data_server.expect_request('/fsd_binary/requiredskillsfortypes.json').respond_with_data(string_data.requiredskillsfortypes)
        self.__data_server.expect_request('/fsd_binary/dynamicitemattributes.json').respond_with_data(string_data.dynamicitemattributes)
        # Get request and send it
        req = self.create_source_request(data=data)
        resp = self.__session.send(req.prepare())
        assert resp.status_code == 201


    def create_sources(self):
        for data in self.__datas.values():
            self.create_source(data)
