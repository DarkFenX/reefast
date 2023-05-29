from tests.support.util import conditional_insert


class Modifier:

    def __init__(self, func, domain, src_attr_id, tgt_attr_id, operation):
        self.func = func
        self.domain = domain
        self.src_attr_id = src_attr_id
        self.tgt_attr_id = tgt_attr_id
        self.operation = operation

    def to_primitives(self):
        mod_entry = {}
        conditional_insert(mod_entry, 'func', self.func)
        conditional_insert(mod_entry, 'domain', self.domain)
        conditional_insert(mod_entry, 'modifyingAttributeID', self.src_attr_id)
        conditional_insert(mod_entry, 'modifiedAttributeID', self.tgt_attr_id)
        conditional_insert(mod_entry, 'operation', self.operation)
        return mod_entry
