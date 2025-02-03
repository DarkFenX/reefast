#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CState {
    Offline,
    Online,
    Active,
    Overload,
}
impl From<&rc::ad::AState> for CState {
    fn from(a_state: &rc::ad::AState) -> Self {
        match a_state {
            rc::ad::AState::Offline => Self::Offline,
            rc::ad::AState::Online => Self::Online,
            rc::ad::AState::Active => Self::Active,
            rc::ad::AState::Overload => Self::Overload,
        }
    }
}
impl From<&CState> for rc::ad::AState {
    fn from(c_state: &CState) -> Self {
        match c_state {
            CState::Offline => Self::Offline,
            CState::Online => Self::Online,
            CState::Active => Self::Active,
            CState::Overload => Self::Overload,
        }
    }
}
