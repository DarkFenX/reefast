#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatTank<T>
where
    T: serde::Serialize,
{
    shield: T,
    armor: T,
    hull: T,
}
impl<T> HStatTank<Option<T>>
where
    T: serde::Serialize,
{
    pub(crate) fn from_opt<U>(core_stat: rc::stats::StatTank<Option<U>>) -> Self
    where
        U: Into<T>,
    {
        Self {
            shield: core_stat.shield.map(|v| v.into()),
            armor: core_stat.armor.map(|v| v.into()),
            hull: core_stat.hull.map(|v| v.into()),
        }
    }
}
impl<T, U> From<rc::stats::StatTank<U>> for HStatTank<T>
where
    T: serde::Serialize,
    U: Into<T>,
{
    fn from(core_stat: rc::stats::StatTank<U>) -> Self {
        Self {
            shield: core_stat.shield.into(),
            armor: core_stat.armor.into(),
            hull: core_stat.hull.into(),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatLayerHp {
    buffer: rc::AttrVal,
    ancil_local: rc::AttrVal,
    ancil_remote: rc::AttrVal,
}
impl From<rc::stats::StatLayerHp> for HStatLayerHp {
    fn from(core_stat: rc::stats::StatLayerHp) -> Self {
        Self {
            buffer: core_stat.buffer,
            ancil_local: core_stat.ancil_local,
            ancil_remote: core_stat.ancil_remote,
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatLayerEhp {
    buffer: rc::AttrVal,
    ancil_local: rc::AttrVal,
    ancil_remote: rc::AttrVal,
    mult: rc::AttrVal,
}
impl From<rc::stats::StatLayerEhp> for HStatLayerEhp {
    fn from(core_stat: rc::stats::StatLayerEhp) -> Self {
        Self {
            buffer: core_stat.buffer,
            ancil_local: core_stat.ancil_local,
            ancil_remote: core_stat.ancil_remote,
            mult: core_stat.mult,
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatLayerRps {
    local: rc::AttrVal,
    remote: rc::AttrVal,
    remote_penalized: rc::AttrVal,
}
impl From<rc::stats::StatLayerRps> for HStatLayerRps {
    fn from(core_stat: rc::stats::StatLayerRps) -> Self {
        Self {
            local: core_stat.local,
            remote: core_stat.remote,
            remote_penalized: core_stat.remote_penalized,
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatLayerErps {
    local: rc::AttrVal,
    remote: rc::AttrVal,
    remote_penalized: rc::AttrVal,
    mult: rc::AttrVal,
}
impl From<rc::stats::StatLayerErps> for HStatLayerErps {
    fn from(core_stat: rc::stats::StatLayerErps) -> Self {
        Self {
            local: core_stat.local,
            remote: core_stat.remote,
            remote_penalized: core_stat.remote_penalized,
            mult: core_stat.mult,
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatLayerResist {
    em: rc::AttrVal,
    thermal: rc::AttrVal,
    kinetic: rc::AttrVal,
    explosive: rc::AttrVal,
}
impl From<rc::stats::DmgKinds<rc::AttrVal>> for HStatLayerResist {
    fn from(core_stat: rc::stats::DmgKinds<rc::AttrVal>) -> Self {
        Self {
            em: core_stat.em,
            thermal: core_stat.thermal,
            kinetic: core_stat.kinetic,
            explosive: core_stat.explosive,
        }
    }
}
